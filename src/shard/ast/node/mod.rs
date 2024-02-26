pub use markdown::unist::Position;
pub mod arena;

pub use arena::{ArenaNode, ArenaAst};

#[macro_export]
/// Automatically implements all node builders per-type
macro_rules! new_nodes_types {
    () => {
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
}

#[macro_export]
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

#[macro_export]
/// Convert Markdown-rs nodes into internal nodes (Ast or Free)
macro_rules! convert {
    (($node_type:ident, $arena:ident, $node:ident) {$($typ:ident => $body:tt;)+}) => {
        match $node {
            $(mdast::Node:: $typ ($node) => convert!($node_type, $arena, $node, $typ, $body)),+
        }
    };

    ($node_type:ident, $arena:ident, $node:ident, $typ:ident, ()) => {
        {
            let children = convert_raw_nodes($arena, $node.children);
            let node = paste!($node_type::[<new_ $typ:snake>](children, $node.position));
            Some($arena.insert(node))
        }
    };

    ($node_type:ident, $arena:ident, $node:ident, $typ:ident, (no_children)) => {
        {
            let children = convert_raw_nodes($arena, Vec::default());
            let node = paste!($node_type::[<new_ $typ:snake>](children, $node.position));
            Some($arena.insert(node))
        }
    };

    ($node_type:ident, $arena:ident, $node:ident, $typ:ident, [$($attr:ident),+]) => {
        paste! {
            {
                let children = convert_raw_nodes($arena, $node.children);
                let node = $node_type::[<new_ $typ:snake>](
                    $typ {
                        $($attr: $node.$attr),+
                    },
                    children,
                    $node.position
                );
                Some($arena.insert(node))
            }
        }
    };


    ($node_type:ident, $arena:ident, $node:ident, $typ:ident, (no_children, [$($attr:ident),+])) => {
        paste! {
            {
                let children = convert_raw_nodes($arena, Vec::default());
                let node = $node_type::[<new_ $typ:snake>](
                    $typ {
                        $($attr: $node.$attr),+
                    },
                    children,
                    $node.position
                );
                Some($arena.insert(node))
            }
        }
    };

    ($node_type:ident, $arena:ident, $node:ident, $typ:ident, (no_children, $attr:expr)) => {
        paste! {
            {
                let children = convert_raw_nodes($arena, Vec::default());
                let node = $node_type::[<new_ $typ:snake>](
                    $attr,
                    children,
                    $node.position
                );
                Some($arena.insert(node))
            }
        }
    };

    ($node_type:ident, $arena:ident, $node:ident, $typ:ident, $attr:expr) => {
        paste! {
            {
                let children = convert_raw_nodes($arena, Vec::default());
                let node = $node_type::[<new_ $typ:snake>](
                    $attr,
                    children,
                    $node.position
                );
                Some($arena.insert(node))
            }
        }
    };
}

mod traits {
    pub trait Node {
        /// Returns the node type.
        fn get_type(&self) -> super::NodeType;
        /// Returns the node attributes.
        fn get_attributes(&self) -> super::NodeAttributes;
        /// Returns the position of the node in the document.
        fn get_position(&self) -> Option<&super::Position>;
    }

    pub trait NodeExplorer<'node> {
        type Node: 'static;
        type Iterator: Iterator<Item=&'node Self::Node> + 'node;
    
        /// Iterate over children
        fn iter_children(&'node self, node: &'node Self::Node) -> Self::Iterator;
    }

    /// Node converter from markdown-rs node to internal node type (Free or Ast)
    pub trait NodeConverterStrategy {
        pub type Node;
        pub type NodeRef;
        
        /// Insert node in the tree.
        fn insert_node(&mut self, parent: Option<Self::NodeRef>, node: Self::Node);

        fn convert_children(&mut self, children: Vec<markdown::mdast::Node>) -> Vec<Self::NodeRef> {
            let mut converted_children: Vec<Self::NodeRef> = vec![];

            for child in children.into_iter() {
               convert_children.push(self.convert(child));
            }

            converted_children.into_iter().flatten().collect()
        }

        /// Convert the node.
        fn convert(&mut self, node: markdown::mdast::Node) -> Self::NodeRef;
    }
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
