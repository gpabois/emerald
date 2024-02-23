pub use markdown::unist::Position;
use markdown::mdast;
use paste::paste;

pub type NodeIndex = generational_arena::Index;

pub struct Node {
    position: Option<Position>,
    children: Vec<NodeIndex>,
    attributes: NodeAttributes,
    r#type: NodeType
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
    new_node_type!{Root}
    new_node_type!{BlockQuote}
    new_node_type!{FootnodeDefinition, FootnodeDefinition}
    new_node_type!{FootnodeReference, FootnodeReference}
    new_node_type!{MdxJsxFlowElement, MdxJsxFlowElement}
    new_node_type!{MdxFlowExpression, MdxFlowExpression}
    new_node_type!{MdxJsElement, MdxJsElement}
    new_node_type!{MdxJsTextElement, MdxJsTextElement}
    new_node_type!{MdxTextExpression, MdxTextExpression}
    new_node_type!{List, List}
    new_node_type!{ListItem, ListItem}
    new_node_type!{Yaml, serde_yaml::Value}
    new_node_type!{Toml, toml::Value}
    new_node_type!{Json, serde_json::Value}
    new_node_type!{Html, String}
    new_node_type!{ThematicBreak}
    new_node_type!{Break}
    new_node_type!{InlineCode, InlineCode}
    new_node_type!{InlineMath, InlineMath}
    new_node_type!{Text, String}
    new_node_type!{Delete}
    new_node_type!{Emphasis}
    new_node_type!{Strong}
    new_node_type!{Image, Image}
    new_node_type!{ImageReference, ImageReference}
    new_node_type!{Link, Link}
    new_node_type!{LinkReference, LinkReference}
    new_node_type!{Code, Code}
    new_node_type!{Math, Math}
    new_node_type!{Heading, Heading}
    new_node_type!{Table, Table}
    new_node_type!{TableRow}
    new_node_type!{TableCell}
    new_node_type!{Paragraph}
}

pub enum NodeAttributes {
    Root,
    BlockQuote,
    FootnodeDefinition(FootnodeDefinition),
    FootnodeReference(FootnodeReference),
    MdxJsxFlowElement(MdxJsxFlowElement),
    MdxFlowExpression(MdxFlowExpression),
    MdxJsElement(MdxJsElement),
    MdxJsTextElement(MdxJsTextElement),
    MdxTextExpression(MdxTextExpression),
    List(List),
    ListItem(ListItem),
    Yaml(serde_yaml::Value),
    Toml(toml::Value),
    Json(serde_json::Value),
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
    Table(Table),
    TableRow,
    TableCell,
    Paragraph
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
    Paragraph
}
pub struct FootnodeDefinition {
    pub identifier: String,
    pub label: Option<String>
}

pub struct FootnodeReference {
    pub identifier: String,
    pub label: Option<String>
}

/// MDX: JSX Element
/// Ex: <tag />
pub struct MdxJsxFlowElement {
    pub name: Option<String>,
    pub attributes: Vec<markdown::mdast::AttributeContent>
}

pub struct MdxFlowExpression {
    pub value: String
}

pub struct MdxJsElement {
    pub value: String
}

pub struct MdxJsTextElement {
    pub name: Option<String>,
    pub attributes: Vec<markdown::mdast::AttributeContent>
}

pub struct MdxTextExpression {
    pub value: String
}

pub struct List {
    pub ordered: bool,
    pub start: Option<u32>,
    pub spread: bool
}

pub struct ListItem {
    pub checked: bool,
    pub spread: bool
}

pub struct InlineCode {
    pub value: String
}

pub struct InlineMath {
    pub value: String
}

pub struct Image {
    pub alt: String,
    pub url: String,
    pub title: Option<String>
}

pub struct ImageReference {
    pub alt: String,
    pub reference_kind: markdown::mdast::ReferenceKind,
    pub label: Option<String>
}

pub struct Link {
    pub url: String,
    pub title: Option<String>
}

pub struct LinkReference {
    pub reference_kind: markdown::mdast::ReferenceKind,
    pub identifier: String,
    pub label: Option<String>
}

pub struct Code {
    pub value: String,
    pub lang: Option<String>,
    pub meta: Option<String>
}

pub struct Math {
    pub value: String,
    pub meta: Option<String>
}

pub struct Heading {
    depth: u8
}

pub struct Table {
    align: Vec<markdown::mdast::AlignKind>
}

pub struct Definition {
    pub url: String,
    pub title: Option<String>,
    pub identifier: String,
    pub label: Option<String>
}
