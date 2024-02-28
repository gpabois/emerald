use std::{fmt::Write, iter::{zip, Zip}};
use markdown::mdast::{Table, TableCell, TableRow, Text};

use super::{r#ref::NodeRef, Ast};

impl std::fmt::Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_node_refs(f, self.walk_ref().max_depth(0))
    }
}

fn fmt_node_refs<'tree, W: Write>(f: &mut W, nodes: impl Iterator<Item=NodeRef<'tree>>) -> std::fmt::Result {
    nodes.map(|node| write!(f, "{}", node)).collect::<Result<Vec<_>, _>>()?;
    Ok(())
}

impl<'tree> std::fmt::Display for NodeRef<'tree> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.attributes.clone() {
            super::NodeAttributes::Root => fmt_node_refs(f, self.iter_children()),
            super::NodeAttributes::BlockQuote => {
                let mut body = String::default();
                fmt_node_refs(&mut body, self.iter_children())?;
                body = body.split('\n').map(|line| format!("> {}", line)).collect::<String>();
                write!(f, "{}", body)
            },
            super::NodeAttributes::FootnoteDefinition(attrs) => {
                write!(
                    f, 
                    "[^{label_or_identifier}]: ", 
                    label_or_identifier = [attrs.label, Some(attrs.identifier)].into_iter().flatten().next().unwrap_or_default()
                )?;
                fmt_node_refs(f, self.iter_children())
            },
            super::NodeAttributes::FootnoteReference(attrs) => {
                write!(f, "[^{label_or_identifier}]", label_or_identifier = [attrs.label, Some(attrs.identifier)].into_iter().flatten().next().unwrap_or_default())
            },
            super::NodeAttributes::MdxJsxFlowElement(_) => fmt_node_refs(f, self.iter_children()),
            super::NodeAttributes::MdxFlowExpression(_) => fmt_node_refs(f, self.iter_children()),
            super::NodeAttributes::MdxjsEsm(_) => fmt_node_refs(f, self.iter_children()),
            super::NodeAttributes::MdxJsxTextElement(_) => fmt_node_refs(f, self.iter_children()),
            super::NodeAttributes::MdxTextExpression(_) => fmt_node_refs(f, self.iter_children()),
            super::NodeAttributes::List(_) => fmt_node_refs(f, self.iter_children()),
            super::NodeAttributes::ListItem(_) => fmt_node_refs(f, self.iter_children()),
            super::NodeAttributes::FrontMatter(frontmatter) => {
                writeln!(f, "---")?;
                write!(f, "{}", frontmatter)?;
                writeln!(f, "---")
            },
            super::NodeAttributes::Html(value) => write!(f, "{}", value),
            super::NodeAttributes::ThematicBreak => writeln!(f, "***"),
            super::NodeAttributes::Break => writeln!(f, "\\"),
            super::NodeAttributes::InlineCode(attrs) => {
                write!(f, "`{}`", attrs.value)
            },
            super::NodeAttributes::InlineMath(attrs) => {
                write!(f, "${}$", attrs.value)
            },
            super::NodeAttributes::Text(value) => write!(f, "{}", value),
            super::NodeAttributes::Delete => {
                write!(f, "~~")?;
                fmt_node_refs(f, self.iter_children())?;
                write!(f, "~~")
            },
            super::NodeAttributes::Emphasis => {
                write!(f, "*")?;
                fmt_node_refs(f, self.iter_children())?;
                write!(f, "*")
            },
            super::NodeAttributes::Strong => {
                write!(f, "**")?;
                fmt_node_refs(f, self.iter_children())?;
                write!(f, "**")
            },
            super::NodeAttributes::Image(attrs) => {
                write!(f, "![{}]", attrs.alt)?;
                write!(f, "(\"{}\"", attrs.url)?;
                if let Some(title) = attrs.title {
                    write!(f, " \"{}\"", title)?;
                }
                write!(f, ")")
            },
            super::NodeAttributes::ImageReference(attrs) => {
                write!(f, "!")?;

                if !attrs.alt.is_empty() {
                    write!(f, "[{}]", attrs.alt)?;
                }

                write!(f, "[{}", attrs.identifier)?;

                if let Some(title) = attrs.label {
                    write!(f, " \"{}\"", title)?;
                }

                write!(f, "]")
            },
            super::NodeAttributes::Link(attrs) => {
                if self.children.is_empty() {
                    write!(f, "[")?;
                    fmt_node_refs(f, self.iter_children())?;
                    write!(f, "]({}", attrs.url)?;
                    if let Some(title) = attrs.title {
                        write!(f, " \"{}\"", title)?;
                    }
                    write!(f, ")")
                } else {
                    write!(f, "[{}]", attrs.url)
                }
            },
            super::NodeAttributes::LinkReference(attrs) => {
                write!(f, "[{}]", attrs.identifier)?;

                if let Some(label) = attrs.label {
                    write!(f, "[{}]", label)?;
                }

                Ok(())
            },
            super::NodeAttributes::Code(attrs) => {
                write!(f, "```")?;
                
                if let Some(lang) = attrs.lang {
                    write!(f, "{} ", lang)?;
                }

                if let Some(meta) = attrs.meta {
                    write!(f, "\"{}\"", meta)?;
                }
                
                writeln!(f, "")?;
                writeln!(f, "{}", attrs.value)?;
                write!(f, "```")?;

                Ok(())
            },
            super::NodeAttributes::Math(attrs) => {
                writeln!(f, "$$")?;
                writeln!(f, "{}", attrs.value)?;
                writeln!(f, "$$")
            },
            super::NodeAttributes::Heading(attrs) => {
                write!(f, "{} ", "#".repeat(attrs.depth.into()))?;
                fmt_node_refs(f, self.iter_children())
            },
            super::NodeAttributes::Definition(attrs) => {
                write!(f, "[{}]: {}", attrs.identifier, attrs.url)
            },
            super::NodeAttributes::Table(_) => todo!(),
            super::NodeAttributes::TableRow => todo!(),
            super::NodeAttributes::TableCell => todo!(),

            super::NodeAttributes::Paragraph => fmt_node_refs(f, self.iter_children()),
        }
    }
}

#[derive(Default)]
struct Box {
    /// (W, H)
    content: (usize, usize),
    border:  (usize, usize)
}

impl Box {
    pub fn outer(&self) -> (usize, usize) {
        [self.content, self.border].iter().fold((0, 0), |(w, h), &(dw, dh)| {
            (
                w+dw,
                h+dh
            )
        })
    }
}

struct TextFrame {
    r#box: Box,
    text: String,
}

impl TextFrame {
    fn new(text: String) -> Self {
        Self {
            r#box: Box::default(),
            text: text
        }
    }

    fn fit(&mut self) {
        self.r#box.content = (
            self.text.len(),
            1
        )
    }
}

struct TableCellFrame {
    r#box: Box,
    children: Vec<TextFrame>
}

impl TableCellFrame {
    fn new(node: NodeRef<'_>) -> Self {
        match node.attributes {
            super::NodeAttributes::TableCell => {
                let mut inner: String = "".into();
                write!(&mut inner, "{}", node).unwrap();
                
                let children = inner.split("\n").map(|line| line.to_string()).map(TextFrame::new).collect::<Vec<_>>();
                Self {
                    r#box: Box::default(),
                    children
                }
            },
            _ => panic!("not a table cell")
        }
    }

    /// Fit the block depending on its descendants
    pub fn fit(&mut self) {
        self.r#box.content = self.children.iter().fold((0, 0), |(w, h), b| {
            let (cw, ch) = b.r#box.outer();

            (
                w + cw,
                std::cmp::max(h, ch)
            )
        })
    }
}

struct TableRowFrame {
    r#box: Box,
    children: Vec<TableCellFrame>
}

impl TableRowFrame {
    fn new(node: NodeRef<'_>) -> Self {
        match node.attributes {
            super::NodeAttributes::TableCell => {
                Self {
                    r#box: Box::default(),
                    children: node.iter_children().map(TableCellFrame::new).collect()
                }
            },
            _ => panic!("not a table row")
        }
    }

    fn layout(&mut self) {
        self.r#box.content = self.children.iter().map(|c| c.r#box.outer()).fold((0, 0), |(w, h), (dw, dh)| {
            (
                w + dw,
                std::cmp::max(h, dh)
            )
        })
    }
}

struct TableFrame {
    r#box: Box,
    children: Vec<TableRowFrame>
}

#[derive(Default)]
struct TableColumn<'a>(Vec<&'a TableCellFrame>);

impl<'a> TableColumn<'a> {
    fn iter(&self) -> impl Iterator<Item=&'a TableCellFrame> {
        self.0.clone().into_iter()
    }
}

impl<'a> FromIterator<&'a TableCellFrame> for TableColumn<'a> {
    fn from_iter<T: IntoIterator<Item = &'a TableCellFrame>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl TableFrame {
    fn new(node: NodeRef<'_>) -> Self {
        match node.attributes {
            super::NodeAttributes::TableCell => {
                Self {
                    r#box: Box::default(),
                    children: node.iter_children().map(TableRowFrame::new).collect()
                }
            },
            _ => panic!("not a table")
        }
    }

    /// Returns (nb_rows, nb_cols)
    pub fn dimensions(&self) -> (usize, usize) {
        let nb_rows = self.children.len();
        let nb_columns = self.children.iter().map(|row| row.children.len()).max().unwrap_or_default();
        (nb_rows, nb_columns)
    }

    pub fn iter_columns(&self) -> impl Iterator<Item=TableColumn<'_>> {
        let (_, nb_cols) = self.dimensions();

        (0..nb_cols).map(|col| {
            self.children.iter().map(|row| &row.children[col]).collect()
        })
    }

    /// Layout the table
    pub fn layout(&mut self) {
        // Fit every cells
        self.children.iter_mut().for_each(|row| row.children.iter_mut().for_each(TableCellFrame::fit));

        // For every column, find the maximum content width
        let max_widths = self.iter_columns().map(|col| col.iter().map(|cell| cell.r#box.content.0).max().unwrap_or_default()).collect::<Vec<_>>();

        // Resize the cell frame's content width accordingly.
        for (col_index, width) in max_widths.iter().enumerate() {
            self.children.iter_mut().for_each(|row| row.children[0].r#box.content.0 = *width);
        }

        // Layout the rows
        self.children.iter_mut().for_each(|row| row.layout())
    }
}