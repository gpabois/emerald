pub use markdown::unist::Position;
use paste::paste;

use super::NodeArena;

pub type NodeIndex = generational_arena::Index;

pub struct Node {
    position: Option<Position>,
    children: Vec<NodeIndex>,
    attributes: NodeAttributes,
    r#type: NodeType,
}

impl Node {
    pub fn iter_children<'a>(&'a self, arena: &'a NodeArena) -> impl Iterator<Item = &Node> + 'a {
        self.children.iter().flat_map(|index| arena.get(*index))
    }

    pub fn children_to_string(&self, arena: &NodeArena) -> String {
        self.iter_children(arena)
            .map(|child| child.to_string(arena))
            .collect()
    }

    pub fn to_string(&self, arena: &NodeArena) -> String {
        match self.attributes {
            NodeAttributes::Root => self
                .iter_children(arena)
                .map(|child| child.to_string(arena))
                .collect(),
            NodeAttributes::BlockQuote => self
                .iter_children(arena)
                .map(|child| child.to_string(arena))
                .collect::<String>()
                .split('\n')
                .map(|line| format!("> {}", line))
                .collect(),
            NodeAttributes::FootnoteDefinition(attrs) => {
                format!(
                    "[^{label}]: {children}",
                    label = attrs.identifier,
                    children = self.children_to_string(arena)
                )
            }
            NodeAttributes::FootnoteReference(attrs) => {
                format!("[{label}]", label = attrs.identifier)
            }
            NodeAttributes::MdxJsxFlowElement(attrs) => {
                let props = attrs
                    .attributes
                    .iter()
                    .map(|attr| match attr {
                        markdown::mdast::AttributeContent::Expression { value, stops } => {
                            format!("{{{}}}", value)
                        }
                        markdown::mdast::AttributeContent::Property(prop) => {
                            let rhs = if let Some(value) = prop.value {
                                match value {
                                    markdown::mdast::AttributeValue::Expression(expr) => {
                                        format!("={{{}}}", expr.value)
                                    }
                                    markdown::mdast::AttributeValue::Literal(lit) => {
                                        format!("=\"{}\"", lit)
                                    }
                                }
                            } else {
                                "".into()
                            };

                            format!("{}{}", prop.name, rhs)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ");

                if self.children.is_empty() {
                    format!("<{} {}/>", attrs.name.unwrap_or_default(), props)
                } else {
                    format!(
                        "<{name} {props}></{name}>",
                        name = attrs.name.unwrap_or_default(),
                        props = props
                    )
                }
            }
            NodeAttributes::MdxFlowExpression(attrs) => format!("{{{}}}", attrs.value),
            NodeAttributes::MdxjsEsm(attrs) => attrs.value,
            NodeAttributes::MdxJsxTextElement(attrs) => {
                let props = attrs
                    .attributes
                    .iter()
                    .map(|attr| match attr {
                        markdown::mdast::AttributeContent::Expression { value, stops } => {
                            format!("{{{}}}", value)
                        }
                        markdown::mdast::AttributeContent::Property(prop) => {
                            let rhs = if let Some(value) = prop.value {
                                match value {
                                    markdown::mdast::AttributeValue::Expression(expr) => {
                                        format!("={{{}}}", expr.value)
                                    }
                                    markdown::mdast::AttributeValue::Literal(lit) => {
                                        format!("=\"{}\"", lit)
                                    }
                                }
                            } else {
                                "".into()
                            };

                            format!("{}{}", prop.name, rhs)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ");

                if self.children.is_empty() {
                    format!("<{} {}/>", attrs.name.unwrap_or_default(), props)
                } else {
                    format!(
                        "<{name} {props}></{name}>",
                        name = attrs.name.unwrap_or_default(),
                        props = props
                    )
                }
            }
            NodeAttributes::MdxTextExpression(attrs) => format!("{{{}}}", attrs.value),
            NodeAttributes::List(_) => self.children_to_string(arena),
            NodeAttributes::ListItem(attrs) => self.children_to_string(arena),
            NodeAttributes::Yaml(value) => {
                let ser = serde_yaml::to_string(&value).unwrap();
                vec!["---".into(), ser, "---".into()].join("\n")
            }
            NodeAttributes::Toml(value) => {
                let ser = toml::to_string(&value).unwrap();
                vec!["---".into(), ser, "---".into()].join("\n")
            }
            NodeAttributes::Html(value) => value,
            NodeAttributes::ThematicBreak => "***".into(),
            NodeAttributes::Break => "\\".into(),
            NodeAttributes::InlineCode(attrs) => format!("´{}´", attrs.value),
            NodeAttributes::InlineMath(attrs) => format!("${}$", attrs.value),
            NodeAttributes::Text(text) => text,
            NodeAttributes::Delete => format!("~{}~", self.children_to_string(arena)),
            NodeAttributes::Emphasis => format!("*{}*", self.children_to_string(arena)),
            NodeAttributes::Strong => format!("**{}**", self.children_to_string(arena)),
            NodeAttributes::Image(attrs) => format!(
                "![{alt}](\"{url}\"{title})",
                alt = attrs.alt,
                url = attrs.url,
                title = attrs
                    .title
                    .map(|title| format!(" \"{}\"", title))
                    .unwrap_or_default()
            ),
            NodeAttributes::ImageReference(attrs) => format!(
                "![{alt}][{identifier}{title}]",
                alt = attrs.alt,
                identifier = attrs.identifier,
                title = attrs
                    .label
                    .map(|title| format!(" \"{}\"", title))
                    .unwrap_or_default()
            ),
            NodeAttributes::Link(attrs) => {
                if let Some(title) = attrs.title {
                    format!("[{title}]({url})", title = title, url = attrs.url)
                } else {
                    format!("[{url}]", url = attrs.url)
                }
            }
            NodeAttributes::LinkReference(attrs) => todo!(),
            NodeAttributes::Code(_) => todo!(),
            NodeAttributes::Math(_) => todo!(),
            NodeAttributes::Heading(_) => todo!(),
            NodeAttributes::Definition(_) => todo!(),
            NodeAttributes::Table(_) => todo!(),
            NodeAttributes::TableRow => todo!(),
            NodeAttributes::TableCell => todo!(),
            NodeAttributes::Paragraph => todo!(),
        }
    }
}

macro_rules! new_node_type {
    ($type:ident) => {
        paste! {
            pub fn [<new_ $type:snake>] <Children>(children: Children, position: Option<Position>) -> Self
            where Children: Iterator<Item=NodeIndex>
            {
                Self {
                    position,
                    children: children.collect(),
                    r#type: NodeType::$type,
                    attributes: NodeAttributes::$type
                }
            }
        }

    };
    ($type:ident, $attributes:path) => {
        paste! {
            pub fn [<new_ $type:snake>]<Children>(attributes: $attributes, children: Children, position: Option<Position>) -> Self
            where Children: Iterator<Item=NodeIndex>
            {
                Self {
                    position,
                    children: children.collect(),
                    r#type: NodeType::$type,
                    attributes: NodeAttributes::$type(attributes)
                }
            }
        }
    };
}

impl Node {
    new_node_type! {Root}
    new_node_type! {BlockQuote}
    new_node_type! {FootnoteDefinition, FootnoteDefinition}
    new_node_type! {FootnoteReference, FootnoteReference}
    new_node_type! {MdxJsxFlowElement, MdxJsxFlowElement}
    new_node_type! {MdxFlowExpression, MdxFlowExpression}
    new_node_type! {MdxjsEsm, MdxjsEsm}
    new_node_type! {MdxJsxTextElement, MdxJsxTextElement}
    new_node_type! {MdxTextExpression, MdxTextExpression}
    new_node_type! {List, List}
    new_node_type! {ListItem, ListItem}
    new_node_type! {Yaml, serde_yaml::Value}
    new_node_type! {Toml, toml::Value}
    new_node_type! {Html, String}
    new_node_type! {ThematicBreak}
    new_node_type! {Break}
    new_node_type! {InlineCode, InlineCode}
    new_node_type! {InlineMath, InlineMath}
    new_node_type! {Text, String}
    new_node_type! {Delete}
    new_node_type! {Emphasis}
    new_node_type! {Strong}
    new_node_type! {Image, Image}
    new_node_type! {ImageReference, ImageReference}
    new_node_type! {Link, Link}
    new_node_type! {LinkReference, LinkReference}
    new_node_type! {Code, Code}
    new_node_type! {Math, Math}
    new_node_type! {Heading, Heading}
    new_node_type! {Definition, Definition}
    new_node_type! {Table, Table}
    new_node_type! {TableRow}
    new_node_type! {TableCell}
    new_node_type! {Paragraph}
}

pub enum NodeAttributes {
    Root,
    BlockQuote,
    FootnoteDefinition(FootnoteDefinition),
    FootnoteReference(FootnoteReference),
    MdxJsxFlowElement(MdxJsxFlowElement),
    MdxFlowExpression(MdxFlowExpression),
    MdxjsEsm(MdxjsEsm),
    MdxJsxTextElement(MdxJsxTextElement),
    MdxTextExpression(MdxTextExpression),
    List(List),
    ListItem(ListItem),
    Yaml(serde_yaml::Value),
    Toml(toml::Value),
    Html(String),
    ThematicBreak,
    Break,
    InlineCode(InlineCode),
    InlineMath(InlineMath),
    Text(String),
    Delete,
    Emphasis,
    Strong,
    Image(Image),
    ImageReference(ImageReference),
    Link(Link),
    LinkReference(LinkReference),
    Code(Code),
    Math(Math),
    Heading(Heading),
    Definition(Definition),
    Table(Table),
    TableRow,
    TableCell,
    Paragraph,
}

pub enum NodeType {
    Root,
    BlockQuote,
    FootnoteDefinition,
    MdxJsxFlowElement,
    List,
    ListItem,
    MdxjsEsm,
    Toml,
    Yaml,
    Break,
    InlineCode,
    InlineMath,
    Delete,
    Emphasis,
    MdxTextExpression,
    FootnoteReference,
    Html,
    Image,
    ImageReference,
    MdxJsxTextElement,
    Link,
    LinkReference,
    Strong,
    Text,
    Math,
    MdxFlowExpression,
    Heading,
    Table,
    TableRow,
    TableCell,
    ThematicBreak,
    Definition,
    Paragraph,
    Code,
}
pub struct FootnoteDefinition {
    pub identifier: String,
    pub label: Option<String>,
}

pub struct FootnoteReference {
    pub identifier: String,
    pub label: Option<String>,
}

/// MDX: JSX Element
/// Ex: <tag />
pub struct MdxJsxFlowElement {
    pub name: Option<String>,
    pub attributes: Vec<markdown::mdast::AttributeContent>,
}

pub struct MdxFlowExpression {
    pub value: String,
}

pub struct MdxjsEsm {
    pub value: String,
}

pub struct MdxJsxTextElement {
    pub name: Option<String>,
    pub attributes: Vec<markdown::mdast::AttributeContent>,
}

pub struct MdxTextExpression {
    pub value: String,
}

pub struct List {
    pub ordered: bool,
    pub start: Option<u32>,
    pub spread: bool,
}

#[derive(Clone)]
pub struct ListItem {
    pub checked: Option<bool>,
    pub spread: bool,
}

pub struct InlineCode {
    pub value: String,
}

pub struct InlineMath {
    pub value: String,
}

pub struct Image {
    pub alt: String,
    pub url: String,
    pub title: Option<String>,
}

pub struct ImageReference {
    pub alt: String,
    pub identifier: String,
    pub reference_kind: markdown::mdast::ReferenceKind,
    pub label: Option<String>,
}

pub struct Link {
    pub url: String,
    pub title: Option<String>,
}

pub struct LinkReference {
    pub reference_kind: markdown::mdast::ReferenceKind,
    pub identifier: String,
    pub label: Option<String>,
}

pub struct Code {
    pub value: String,
    pub lang: Option<String>,
    pub meta: Option<String>,
}

pub struct Math {
    pub value: String,
    pub meta: Option<String>,
}

pub struct Heading {
    pub depth: u8,
}

pub struct Table {
    pub align: Vec<markdown::mdast::AlignKind>,
}

pub struct Definition {
    pub url: String,
    pub title: Option<String>,
    pub identifier: String,
    pub label: Option<String>,
}
