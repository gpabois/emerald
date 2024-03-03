use std::error::Error;

use indexmap::IndexMap;
use markdown::mdast;
pub use markdown::unist::Position;
pub mod debug;
pub mod display;
pub mod r#ref;

pub use r#ref::NodeRef;

#[macro_export]
/// Implement node conversion
///
/// ```
/// fn convert(
///        strategy: impl NodeConverterStrategy,
///        node: markdown::mdast::Node
///    ) -> Option<NodeConverterStrategy::NodeRef> {
///     convert!(strategy, node)
/// }
/// ```
macro_rules! convert {
    ($typ:ident, $node:ident, $converter:ident) => {
        {
            let children = $converter.convert_children($node.children);
            let node = paste::paste!(Self::[<from_ $typ:snake>](children, $node.position));
            Some($converter.insert_node(node))
        }
    };

    ($typ:ident, $node:ident, $converter:ident, no_children) => {
        {
            let node = paste::paste!(Self::[<from_ $typ:snake>](vec![], $node.position));
            Some($converter.insert_node(node))
        }
    };

    ($typ:ident, $node:ident, $converter:ident, no_children, [$($attr:ident),+]) => {
        paste::paste! {
            {
                let node = Self::[<from_ $typ:snake>](
                    $crate::shard::ast::$typ {
                        $($attr: $node.$attr),+
                    },
                    vec![],
                    $node.position
                );
                Some($converter.insert_node(node))
            }
        }
    };

   ($typ:ident, $node:ident, $converter:ident, [$($attr:ident),+]) => {
        paste::paste! {
            {
                let children = $converter.convert_children($node.children);
                let node = Self::[<from_ $typ:snake>](
                    $crate::shard::ast::$typ {
                        $($attr: $node.$attr),+
                    },
                    children,
                    $node.position
                );
                Some($converter.insert_node(node))
            }
        }
    };

    ($typ:ident, $node:ident, $converter:ident, no_children, $attr_expr:expr) => {
        paste::paste! {
            {
                let node = Self::[<from_ $typ:snake>](
                    $attr_expr,
                    vec![],
                    $node.position
                );
                Some($converter.insert_node(node))
            }
        }
    };

 }
#[macro_export]
/// Automatically defines all per-node type converter function
/// for NodeConverter trait.
///
/// ´´´
/// impl NodeConverter {
///     [...]
///
///     def_from_node_types!{}
/// }
/// ´´´
macro_rules! def_from_node_types {
    () => {
        def_from_node_type! {Root}
        def_from_node_type! {BlockQuote}
        def_from_node_type! {FootnoteDefinition, $crate::shard::ast::FootnoteDefinition}
        def_from_node_type! {FootnoteReference, $crate::shard::ast::FootnoteReference}
        def_from_node_type! {MdxJsxFlowElement, $crate::shard::ast::MdxJsxFlowElement}
        def_from_node_type! {MdxFlowExpression, $crate::shard::ast::MdxFlowExpression}
        def_from_node_type! {MdxjsEsm, $crate::shard::ast::MdxjsEsm}
        def_from_node_type! {MdxJsxTextElement, $crate::shard::ast::MdxJsxTextElement}
        def_from_node_type! {MdxTextExpression, $crate::shard::ast::MdxTextExpression}
        def_from_node_type! {List, $crate::shard::ast::List}
        def_from_node_type! {ListItem, $crate::shard::ast::ListItem}
        def_from_node_type! {Yaml, serde_yaml::Value}
        def_from_node_type! {Toml, toml::Value}
        def_from_node_type! {Html, String}
        def_from_node_type! {ThematicBreak}
        def_from_node_type! {Break}
        def_from_node_type! {InlineCode, $crate::shard::ast::InlineCode}
        def_from_node_type! {InlineMath, $crate::shard::ast::InlineMath}
        def_from_node_type! {Text, String}
        def_from_node_type! {Delete}
        def_from_node_type! {Emphasis}
        def_from_node_type! {Strong}
        def_from_node_type! {Image, $crate::shard::ast::Image}
        def_from_node_type! {ImageReference, $crate::shard::ast::ImageReference}
        def_from_node_type! {Link, $crate::shard::ast::Link}
        def_from_node_type! {LinkReference, $crate::shard::ast::LinkReference}
        def_from_node_type! {Code, $crate::shard::ast::Code}
        def_from_node_type! {Math, $crate::shard::ast::Math}
        def_from_node_type! {Heading, $crate::shard::ast::Heading}
        def_from_node_type! {Definition, $crate::shard::ast::Definition}
        def_from_node_type! {Table, $crate::shard::ast::Table}
        def_from_node_type! {TableRow}
        def_from_node_type! {TableCell}
        def_from_node_type! {Paragraph}
    };
}

#[macro_export]
/// Create a new node type builder
///
/// ´´´
/// pub trait NodeConverter {
///     // ... //
///     def_from_node_type{Root}
/// }
/// ´´´
macro_rules! def_from_node_type {
    ($typ:ident) => {
        paste::paste! {
            fn [<from_ $typ:snake>](
                children: Vec<Self::NodeRef>,
                position: Option<$crate::shard::ast::node::Position>
            ) -> Self::Node;
        }
    };
    ($typ:ident, $attributes:path) => {
        paste::paste! {
           fn [<from_ $typ:snake>](
               attributes: $attributes,
               children: Vec<Self::NodeRef>,
               position: Option<$crate::shard::ast::node::Position>
            ) -> Self::Node;
        }
    };
}

#[macro_export]
/// Automatically implements all node builders per-type
///
/// ´´´
/// impl Node {
///     from_node_types!()
/// }
/// ´´´
macro_rules! from_node_types {
    () => {
        from_node_type! {Root}
        from_node_type! {BlockQuote}
        from_node_type! {FootnoteDefinition, $crate::shard::ast::FootnoteDefinition}
        from_node_type! {FootnoteReference, $crate::shard::ast::FootnoteReference}
        from_node_type! {MdxJsxFlowElement, $crate::shard::ast::MdxJsxFlowElement}
        from_node_type! {MdxFlowExpression, $crate::shard::ast::MdxFlowExpression}
        from_node_type! {MdxjsEsm, $crate::shard::ast::MdxjsEsm}
        from_node_type! {MdxJsxTextElement, $crate::shard::ast::MdxJsxTextElement}
        from_node_type! {MdxTextExpression, $crate::shard::ast::MdxTextExpression}
        from_node_type! {List, $crate::shard::ast::List}
        from_node_type! {ListItem, $crate::shard::ast::ListItem}

        /// Convert YAML node into a FrontMatter node
        fn from_yaml(
            value: serde_yaml::Value,
            children: Vec<Self::NodeRef>,
            position: Option<$crate::shard::ast::Position>,
        ) -> Self::Node {
            Self::Node {
                position,
                children,
                r#type: $crate::shard::ast::NodeType::FrontMatter,
                attributes: $crate::shard::ast::NodeAttributes::FrontMatter(
                    $crate::shard::ast::FrontMatter::from(value),
                ),
            }
        }

        /// Convert TOML node into a FrontMatter node
        fn from_toml(
            value: toml::Value,
            children: Vec<Self::NodeRef>,
            position: Option<$crate::shard::ast::Position>,
        ) -> Self::Node {
            Self::Node {
                position,
                children,
                r#type: $crate::shard::ast::NodeType::FrontMatter,
                attributes: $crate::shard::ast::NodeAttributes::FrontMatter(
                    $crate::shard::ast::FrontMatter::from(value),
                ),
            }
        }

        from_node_type! {Html, String}
        from_node_type! {ThematicBreak}
        from_node_type! {Break}
        from_node_type! {InlineCode, $crate::shard::ast::InlineCode}
        from_node_type! {InlineMath, $crate::shard::ast::InlineMath}
        from_node_type! {Text, String}
        from_node_type! {Delete}
        from_node_type! {Emphasis}
        from_node_type! {Strong}
        from_node_type! {Image, $crate::shard::ast::Image}
        from_node_type! {ImageReference, $crate::shard::ast::ImageReference}
        from_node_type! {Link, $crate::shard::ast::Link}
        from_node_type! {LinkReference, $crate::shard::ast::LinkReference}
        from_node_type! {Code, $crate::shard::ast::Code}
        from_node_type! {Math, $crate::shard::ast::Math}
        from_node_type! {Heading, $crate::shard::ast::Heading}
        from_node_type! {Definition, $crate::shard::ast::Definition}
        from_node_type! {Table, $crate::shard::ast::Table}
        from_node_type! {TableRow}
        from_node_type! {TableCell}
        from_node_type! {Paragraph}
    };
}

#[macro_export]
/// Create a new specific node type converter function.
///
/// ```
/// impl NodeConverter for Foo {
///     from_node_type{Root}
///     from_node_type{Yaml, serde_yaml::Value}
/// }
/// ```
macro_rules! from_node_type {
    ($type:ident) => {
        paste::paste! {
            fn [<from_ $type:snake>](children: Vec<Self::NodeRef>, position: Option<$crate::shard::ast::Position>) -> Self::Node
            {
                Self::Node {
                    position,
                    children,
                    r#type: $crate::shard::ast::NodeType::$type,
                    attributes: $crate::shard::ast::NodeAttributes::$type
                }
            }
        }

    };
    ($type:ident, $attributes:path) => {
        paste::paste! {
            fn [<from_ $type:snake>](
                attributes: $attributes,
                children: Vec<Self::NodeRef>,
                position: Option<$crate::shard::ast::Position>
            ) -> Self::Node
            {
                Self::Node {
                    position,
                    children,
                    r#type: $crate::shard::ast::NodeType::$type,
                    attributes: $crate::shard::ast::NodeAttributes::$type(attributes)
                }
            }
        }
    };
}

pub mod arena;
pub use arena::{Ast, Node};

mod traits {
    pub trait Node {
        /// Returns the node type.
        fn get_type(&self) -> super::NodeType;

        /// Returns the node attributes.
        fn get_attributes(&self) -> &super::NodeAttributes;

        /// Returns the position of the node in the document.
        fn get_position(&self) -> Option<&super::Position>;
    }

    pub trait NodeExplorer<'node> {
        type Node: 'static;
        type Iterator: Iterator<Item = &'node Self::Node> + 'node;

        /// Iterate over children
        fn iter_children(&'node self, node: &'node Self::Node) -> Self::Iterator;
    }

    /// Node converter from markdown-rs node to internal node type (Free or Ast)
    pub trait NodeConverter {
        type Node;
        type NodeRef;

        /// Insert node in the tree's memory region.
        /// It does not insert it to the tree directly.
        fn insert_node(&mut self, node: Self::Node) -> Self::NodeRef;

        /// Convert a list of node.
        fn convert_children(&mut self, children: Vec<markdown::mdast::Node>) -> Vec<Self::NodeRef> {
            let mut converted_children: Vec<Option<Self::NodeRef>> = vec![];

            for child in children.into_iter() {
                converted_children.push(self.convert(child));
            }

            converted_children.into_iter().flatten().collect()
        }

        /// Convert the node.
        fn convert(&mut self, node: markdown::mdast::Node) -> Option<Self::NodeRef> {
            match node {
                markdown::mdast::Node::Root(attr) => convert!(Root, attr, self),
                markdown::mdast::Node::BlockQuote(attr) => convert!(BlockQuote, attr, self),
                markdown::mdast::Node::FootnoteDefinition(attr) => {
                    convert!(
                        FootnoteDefinition,
                        attr,
                        self,
                        no_children,
                        [identifier, label]
                    )
                }

                markdown::mdast::Node::MdxJsxFlowElement(attr) => {
                    convert!(MdxJsxFlowElement, attr, self, [name, attributes])
                }

                markdown::mdast::Node::List(attr) => {
                    convert!(List, attr, self, [ordered, spread, start])
                }

                markdown::mdast::Node::MdxjsEsm(attr) => {
                    convert!(MdxjsEsm, attr, self, no_children, [value])
                }
                markdown::mdast::Node::Toml(attr) => convert!(
                    Toml,
                    attr,
                    self,
                    no_children,
                    toml::from_str(&attr.value).ok()?
                ),
                markdown::mdast::Node::Yaml(attr) => convert!(
                    Yaml,
                    attr,
                    self,
                    no_children,
                    serde_yaml::from_str(&attr.value).ok()?
                ),
                markdown::mdast::Node::Break(attr) => convert!(Break, attr, self, no_children),
                markdown::mdast::Node::InlineCode(attr) => {
                    convert!(InlineCode, attr, self, no_children, [value])
                }
                markdown::mdast::Node::InlineMath(attr) => {
                    convert!(InlineMath, attr, self, no_children, [value])
                }
                markdown::mdast::Node::Delete(attr) => convert!(Delete, attr, self),
                markdown::mdast::Node::Emphasis(attr) => convert!(Emphasis, attr, self),
                markdown::mdast::Node::MdxTextExpression(attr) => {
                    convert!(MdxTextExpression, attr, self, no_children, [value])
                }
                markdown::mdast::Node::FootnoteReference(attr) => convert!(
                    FootnoteReference,
                    attr,
                    self,
                    no_children,
                    [identifier, label]
                ),

                markdown::mdast::Node::Html(attr) => {
                    convert!(Html, attr, self, no_children, attr.value)
                }
                markdown::mdast::Node::Image(attr) => {
                    convert!(Image, attr, self, no_children, [alt, url, title])
                }
                markdown::mdast::Node::ImageReference(attr) => convert!(
                    ImageReference,
                    attr,
                    self,
                    no_children,
                    [alt, reference_kind, identifier, label]
                ),

                markdown::mdast::Node::MdxJsxTextElement(attr) => {
                    convert!(MdxJsxTextElement, attr, self, [name, attributes])
                }

                markdown::mdast::Node::Link(attr) => convert!(Link, attr, self, [url, title]),

                markdown::mdast::Node::LinkReference(attr) => convert!(
                    LinkReference,
                    attr,
                    self,
                    [reference_kind, identifier, label]
                ),
                markdown::mdast::Node::Strong(attr) => convert!(Strong, attr, self),
                markdown::mdast::Node::Text(attr) => {
                    convert!(Text, attr, self, no_children, attr.value)
                }

                markdown::mdast::Node::Code(attr) => {
                    convert!(Code, attr, self, no_children, [value, lang, meta])
                }

                markdown::mdast::Node::Math(attr) => {
                    convert!(Math, attr, self, no_children, [value, meta])
                }

                markdown::mdast::Node::MdxFlowExpression(attr) => {
                    convert!(MdxFlowExpression, attr, self, no_children, [value])
                }
                markdown::mdast::Node::Heading(attr) => convert!(Heading, attr, self, [depth]),
                markdown::mdast::Node::Table(attr) => convert!(Table, attr, self, [align]),
                markdown::mdast::Node::ThematicBreak(attr) => {
                    convert!(ThematicBreak, attr, self, no_children)
                }
                markdown::mdast::Node::TableRow(attr) => convert!(TableRow, attr, self),
                markdown::mdast::Node::TableCell(attr) => {
                    convert!(TableCell, attr, self, no_children)
                }
                markdown::mdast::Node::ListItem(attr) => {
                    convert!(ListItem, attr, self, [checked, spread])
                }
                markdown::mdast::Node::Definition(attr) => convert!(
                    Definition,
                    attr,
                    self,
                    no_children,
                    [url, title, identifier, label]
                ),
                markdown::mdast::Node::Paragraph(attr) => convert!(Paragraph, attr, self),
            }
        }

        def_from_node_types! {}
    }
}

#[derive(Debug, Clone)]
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
    FrontMatter(FrontMatter),
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

#[derive(Debug, Copy, Clone)]
pub enum NodeType {
    Root,
    FrontMatter,

    Definition,

    BlockQuote,

    FootnoteReference,
    FootnoteDefinition,

    List,
    ListItem,

    MdxJsxTextElement,
    MdxFlowExpression,
    MdxJsxFlowElement,
    MdxTextExpression,
    MdxjsEsm,

    Code,
    InlineCode,
    Math,
    InlineMath,

    ThematicBreak,
    Break,

    Heading,

    Delete,
    Emphasis,
    Strong,
    Text,

    Html,

    Image,
    ImageReference,

    Link,
    LinkReference,

    Table,
    TableRow,
    TableCell,

    Paragraph,
}
#[derive(Debug, Clone)]
pub struct FootnoteDefinition {
    pub identifier: String,
    pub label: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FootnoteReference {
    pub identifier: String,
    pub label: Option<String>,
}

#[derive(Debug, Clone)]
/// MDX: JSX Element
/// Ex: <tag />
pub struct MdxJsxFlowElement {
    pub name: Option<String>,
    pub attributes: Vec<markdown::mdast::AttributeContent>,
}

#[derive(Debug, Clone)]
pub struct MdxFlowExpression {
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct MdxjsEsm {
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct MdxJsxTextElement {
    pub name: Option<String>,
    pub attributes: Vec<markdown::mdast::AttributeContent>,
}

#[derive(Debug, Clone)]
pub struct MdxTextExpression {
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct List {
    pub ordered: bool,
    pub start: Option<u32>,
    pub spread: bool,
}

#[derive(Debug, Clone)]
pub struct ListItem {
    pub checked: Option<bool>,
    pub spread: bool,
}

#[derive(Debug, Clone)]
pub struct InlineCode {
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct InlineMath {
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Image {
    pub alt: String,
    pub url: String,
    pub title: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ImageReference {
    pub alt: String,
    pub identifier: String,
    pub reference_kind: markdown::mdast::ReferenceKind,
    pub label: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Link {
    pub url: String,
    pub title: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LinkReference {
    pub reference_kind: markdown::mdast::ReferenceKind,
    pub identifier: String,
    pub label: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Code {
    pub value: String,
    pub lang: Option<String>,
    pub meta: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Math {
    pub value: String,
    pub meta: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Heading {
    pub depth: u8,
}

#[derive(Debug, Clone)]
pub struct Table {
    pub align: Vec<markdown::mdast::AlignKind>,
}

#[derive(Debug, Clone)]
pub struct Definition {
    pub url: String,
    pub title: Option<String>,
    pub identifier: String,
    pub label: Option<String>,
}
