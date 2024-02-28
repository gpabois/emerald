pub use indexmap::IndexMap;

#[derive(Clone)]
pub enum Number {
    Integer(i64),
    Float(f64)
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Integer(value) => write!(f, "{}", value),
            Number::Float(value) => write!(f, "{}", value),
        }
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl Into<serde_yaml::Number> for Number {
    fn into(self) -> serde_yaml::Number {
        match self {
            Number::Integer(value) => value.into(),
            Number::Float(value) => value.into(),
        }
    }
}

impl From<serde_yaml::Number> for Number {
    fn from(value: serde_yaml::Number) -> Self {
        if let Some(unsigned) = value.as_u64() {
            Self::Integer(unsigned.try_into().unwrap())
        } else if let Some(signed) = value.as_i64() {
            Self::Integer(signed)
        } else {
            Self::Float(value.as_f64().unwrap())
        }
    }
}

#[derive(Clone)]
/// Base object for values
pub enum Value {
    Null,
    Boolean(bool),
    String(String),
    Number(Number),
    Array(Vec<Value>),
    Map(IndexMap<String, Value>)
}

impl Into<serde_yaml::Value> for Value {
    fn into(self) -> serde_yaml::Value {
        match self {
            Value::Null => serde_yaml::Value::Null,
            Value::Boolean(value) => serde_yaml::Value::Bool(value),
            Value::String(value) => serde_yaml::Value::String(value),
            Value::Number(value) => serde_yaml::Value::Number(value.into()),
            Value::Array(value) => serde_yaml::Value::Sequence(value.into_iter().map(Self::into).collect()),
            Value::Map(value) => serde_yaml::Value::Mapping(
                value.into_iter().map(|(k, v)| (k.into(), v.into())).collect()
            ),
        }
    }
}

impl From<serde_yaml::Value> for Value {
    fn from(value: serde_yaml::Value) -> Self {
        match value {
            serde_yaml::Value::Null => Self::Null,
            serde_yaml::Value::Bool(value) => Self::Boolean(value),
            serde_yaml::Value::Number(value) => Self::Number(value.into()),
            serde_yaml::Value::String(value) => Self::String(value),
            serde_yaml::Value::Sequence(value) => Self::Array(value.into_iter().map(Self::from).collect()),
            serde_yaml::Value::Mapping(value) => Self::Map(value.into_iter().map(|(k, v)| (
                k.as_str().unwrap().to_owned(),
                Self::from(v)
            )).collect()),
            serde_yaml::Value::Tagged(_) => panic!("Yaml tagging is not handled"),
        }
    }
}

impl From<toml::Value> for Value {
    fn from(value: toml::Value) -> Self {
        match value {
            toml::Value::String(value) => Self::String(value),
            toml::Value::Integer(value) => Self::Number(value.into()),
            toml::Value::Float(value) => Self::Number(value.into()),
            toml::Value::Boolean(value) => Self::Boolean(value.into()),
            // Don't manage the datetime yet.
            toml::Value::Datetime(value) => Self::String(value.to_string()),
            toml::Value::Array(value) => Self::Array(value.into_iter().map(Self::from).collect()),
            toml::Value::Table(value) => Self::Map(
                value.into_iter()
                .map(|(k, v)| (k, Self::from(v)))
                .collect()
            ),
        }
    }
}