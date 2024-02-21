use markdown::mdast::Node;
use serde_yaml::Value;

#[derive(Debug)]
pub struct Header(Value);

impl Header {
    pub fn from_frontmatter(node: Option<&Node>) -> Option<Self> {
        match node? {
            Node::Yaml(yaml) => Self::from_yaml(&yaml.value),
            _ => None
        }
    }

    fn from_yaml(input: &str) -> Option<Self> {
        serde_yaml::from_str(input).ok().map(|value| Header(value))
    }   
}