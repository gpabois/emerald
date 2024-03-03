#[derive(Debug, Clone)]
/// The format of the frontmatter
pub enum FrontMatterFormat {
    Yaml,
    Toml,
}

impl std::fmt::Display for FrontMatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = crate::shard::Value::Map(self.properties.clone());
        match self.format {
            FrontMatterFormat::Yaml => {
                write!(f, "{}", serde_yaml::to_string(&val).unwrap())
            }
            _ => todo!("Implements Toml serialization for Frontmatter's fmt."),
        }
    }
}

impl From<serde_yaml::Value> for FrontMatter {
    fn from(value: serde_yaml::Value) -> Self {
        Self {
            format: FrontMatterFormat::Yaml,
            value: value.into(),
        }
    }
}

impl From<toml::Value> for FrontMatter {
    fn from(value: toml::Value) -> Self {
        Self {
            format: FrontMatterFormat::Toml,
            value: value.into(),
        }
    }
}
#[derive(Debug, Clone)]
/// Holds the metadata of the shard.
///
/// ```
/// ---
/// title: The title of the shard
/// ---
/// ```
pub struct FrontMatter {
    /// The format to project when the AST is serialized.
    pub format: FrontMatterFormat,
    /// The root value of the frontmatter
    pub properties: IndexMap<String, crate::shard::Value>,
}

impl TryFrom<mdast::Yaml> for FrontMatter {
    type Error = Box<dyn Error>;

    fn try_from(value: mdast::Yaml) -> Result<Self, Self::Error> {
        let yaml = serde_yaml::from_str(&value.value)?;
        let value = crate::shard::Value::from(yaml);

        Ok(Self {
            format: FrontMatterFormat::Yaml,
            properties: value.expect_map(),
        })
    }
}
