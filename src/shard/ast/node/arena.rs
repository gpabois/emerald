use markdown::{mdast, to_mdast, Constructs, ParseOptions};

pub use markdown::unist::Position;

pub type ArenaNodeIndex = generational_arena::Index;

/// An AST owned by an arena.
pub struct ArenaAst {
    arena: NodeArena,
    root: ArenaNodeIndex,
}

impl ArenaAst {
    pub fn from_str(input: &str) -> Option<Self> {
        let constructs = Constructs {
            frontmatter: true,
            ..Constructs::gfm()
        };

        let options = ParseOptions {
            constructs,
            ..ParseOptions::default()
        };

        let tree = to_mdast(input, &options).ok()?;
        let mut arena = generational_arena::Arena::new();
        let root = convert_raw_node(&mut arena, tree)?;
        Some(Self { arena, root })
    }
}

/// A node owned by an arena.
pub struct ArenaNode {
    position: Option<Position>,
    children: Vec<ArenaNodeIndex>,
    attributes: NodeAttributes,
    r#type: NodeType,
}

pub struct ArenaNodeExplorer<'node> {
    arena: &'node ArenaNode
}

impl<'node> super::traits::NodeExplorer<'node> for ArenaNodeExplorer<'node> {
    type Node = ArenaNode;
    type Iterator = Box<dyn Iterator<Item=&'node Self::Node> + 'node>;

    fn iter_children(&'node self, node: &'node ArenaNode) -> Self::Iterator {
        Box::new(
            node.children
            .iter()
            .flat_map(|index| self.arena.get(*index))
        )
    }
}

impl super::traits::Node for ArenaNode {

}

impl ArenaNode {
    pub fn iter_children<'node, Explorer: super::traits::NodeExplorer<'node, Node = Self>>(&'node self, explorer: &Explorer) -> Explorer::Iterator {
        explorer.iter_children(self)
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

impl ArenaNode {
    new_nodes_types!{}
}

pub struct NodeArenaExplorer<'node> {
    arena: &'node NodeArena
}

impl<'node> NodeExplorer<'node> for NodeArenaExplorer<'node> {
    type Node = Node;
    type Iterator = Box<dyn Iterator<Item=&'node Node> + 'node>;

    fn iter_children(&'node self, node: &'node Node) -> Self::Iterator {
        Box::new(
            node.children
            .iter()
            .flat_map(|index| self.arena.get(*index))
        )
    }
}

pub struct ArenaNodeConverter<'node> {
    arena: &'node mut Arena
}

impl<'node> super::traits::NodeConverterStrategy for ArenaNodeConverter<'node> {
    type Node = ArenaNode;
    type NodeRef = ArenaNodeIndex;
    
    /// Insert node in the tree.
    fn insert_node(&mut self, node: Self::Node) -> Self::NodeRef {
        self.arena.insert(node)
    }

    fn convert(&mut self, node: markdown::mdast::Node) -> Option<Self::NodeRef> {
        
    }
}