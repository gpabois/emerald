use std::fmt::Write;

use super::{r#ref::NodeRef, Ast};

impl std::fmt::Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_node_refs(f, self.walk_ref().max_depth(0))
    }
}

fn fmt_node_refs<'tree, W: Write>(
    f: &mut W,
    nodes: impl Iterator<Item = NodeRef<'tree>>,
) -> std::fmt::Result {
    nodes
        .map(|node| write!(f, "{}", node))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(())
}

impl<'tree> std::fmt::Display for NodeRef<'tree> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.attributes.clone() {
            super::NodeAttributes::Root => fmt_node_refs(f, self.iter_children()),
            super::NodeAttributes::BlockQuote => {
                let mut body = String::default();
                fmt_node_refs(&mut body, self.iter_children())?;
                body = body
                    .split('\n')
                    .fold(String::new(), |acc, line| format!("{}\n> {}", acc, line));
                write!(f, "{}", body)
            }
            super::NodeAttributes::FootnoteDefinition(attrs) => {
                write!(
                    f,
                    "[^{label_or_identifier}]: ",
                    label_or_identifier = [attrs.label, Some(attrs.identifier)]
                        .into_iter()
                        .flatten()
                        .next()
                        .unwrap_or_default()
                )?;
                fmt_node_refs(f, self.iter_children())
            }
            super::NodeAttributes::FootnoteReference(attrs) => {
                write!(
                    f,
                    "[^{label_or_identifier}]",
                    label_or_identifier = [attrs.label, Some(attrs.identifier)]
                        .into_iter()
                        .flatten()
                        .next()
                        .unwrap_or_default()
                )
            }
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
            }
            super::NodeAttributes::Html(value) => write!(f, "{}", value),
            super::NodeAttributes::ThematicBreak => writeln!(f, "***"),
            super::NodeAttributes::Break => writeln!(f, "\\"),
            super::NodeAttributes::InlineCode(attrs) => {
                write!(f, "`{}`", attrs.value)
            }
            super::NodeAttributes::InlineMath(attrs) => {
                write!(f, "${}$", attrs.value)
            }
            super::NodeAttributes::Text(value) => write!(f, "{}", value),
            super::NodeAttributes::Delete => {
                write!(f, "~~")?;
                fmt_node_refs(f, self.iter_children())?;
                write!(f, "~~")
            }
            super::NodeAttributes::Emphasis => {
                write!(f, "*")?;
                fmt_node_refs(f, self.iter_children())?;
                write!(f, "*")
            }
            super::NodeAttributes::Strong => {
                write!(f, "**")?;
                fmt_node_refs(f, self.iter_children())?;
                write!(f, "**")
            }
            super::NodeAttributes::Image(attrs) => {
                write!(f, "![{}]", attrs.alt)?;
                write!(f, "(\"{}\"", attrs.url)?;
                if let Some(title) = attrs.title {
                    write!(f, " \"{}\"", title)?;
                }
                write!(f, ")")
            }
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
            }
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
            }
            super::NodeAttributes::LinkReference(attrs) => {
                write!(f, "[{}]", attrs.identifier)?;

                if let Some(label) = attrs.label {
                    write!(f, "[{}]", label)?;
                }

                Ok(())
            }
            super::NodeAttributes::Code(attrs) => {
                write!(f, "```")?;

                if let Some(lang) = attrs.lang {
                    write!(f, "{} ", lang)?;
                }

                if let Some(meta) = attrs.meta {
                    write!(f, "\"{}\"", meta)?;
                }

                writeln!(f)?;
                writeln!(f, "{}", attrs.value)?;
                write!(f, "```")?;

                Ok(())
            }
            super::NodeAttributes::Math(attrs) => {
                writeln!(f, "$$")?;
                writeln!(f, "{}", attrs.value)?;
                writeln!(f, "$$")
            }
            super::NodeAttributes::Heading(attrs) => {
                write!(f, "{} ", "#".repeat(attrs.depth.into()))?;
                fmt_node_refs(f, self.iter_children())
            }
            super::NodeAttributes::Definition(attrs) => {
                write!(f, "[{}]: {}", attrs.identifier, attrs.url)
            }
            super::NodeAttributes::Table(_) => todo!(),
            super::NodeAttributes::TableRow => todo!(),
            super::NodeAttributes::TableCell => todo!(),

            super::NodeAttributes::Paragraph => fmt_node_refs(f, self.iter_children()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error, str::FromStr};

    use crate::shard::ast::Ast;

    #[test]
    pub fn display_ast() -> Result<(), Box<dyn Error>> {
        let content = r#"---
title: My shard
date: 01/01/01
---

# Heading
This is a content [property:: value]
"#;

        let ast = Ast::from_str(content)?;
        println!("{}", ast);
        Ok(())
    }
}
