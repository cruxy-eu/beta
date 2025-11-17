use std::env;
use std::env::VarError;
use std::fmt;

/// Error type for input/output operations
#[derive(Debug)]
pub enum IoError {
    MissingRequired(String),
    ParseError(String),
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IoError::MissingRequired(name) => write!(f, "required input '{}' is missing", name),
            IoError::ParseError(msg) => write!(f, "parse error: {}", msg),
        }
    }
}

impl std::error::Error for IoError {}

/// Trait for types that can be parsed from GitHub Actions input strings
pub trait FromInput: Sized {
    fn from_input(s: &str) -> Result<Self, IoError>;
}

impl FromInput for String {
    fn from_input(s: &str) -> Result<Self, IoError> {
        Ok(s.to_string())
    }
}

impl FromInput for bool {
    fn from_input(s: &str) -> Result<Self, IoError> {
        match s.to_lowercase().as_str() {
            "true" | "1" | "yes" => Ok(true),
            "false" | "0" | "no" | "" => Ok(false),
            _ => Err(IoError::ParseError(format!("Cannot parse '{}' as bool", s))),
        }
    }
}

impl FromInput for i32 {
    fn from_input(s: &str) -> Result<Self, IoError> {
        s.parse().map_err(|e| IoError::ParseError(format!("{}", e)))
    }
}

impl FromInput for u32 {
    fn from_input(s: &str) -> Result<Self, IoError> {
        s.parse().map_err(|e| IoError::ParseError(format!("{}", e)))
    }
}

impl FromInput for i64 {
    fn from_input(s: &str) -> Result<Self, IoError> {
        s.parse().map_err(|e| IoError::ParseError(format!("{}", e)))
    }
}

impl FromInput for u64 {
    fn from_input(s: &str) -> Result<Self, IoError> {
        s.parse().map_err(|e| IoError::ParseError(format!("{}", e)))
    }
}

impl<T: FromInput> FromInput for Option<T> {
    fn from_input(s: &str) -> Result<Self, IoError> {
        if s.is_empty() {
            Ok(None)
        } else {
            T::from_input(s).map(Some)
        }
    }
}

/// Trait for types that can be converted to GitHub Actions output strings
pub trait ToOutput {
    fn to_output(&self) -> String;
}

impl ToOutput for String {
    fn to_output(&self) -> String {
        self.clone()
    }
}

impl ToOutput for &str {
    fn to_output(&self) -> String {
        self.to_string()
    }
}

impl ToOutput for bool {
    fn to_output(&self) -> String {
        self.to_string()
    }
}

impl ToOutput for i32 {
    fn to_output(&self) -> String {
        self.to_string()
    }
}

impl ToOutput for u32 {
    fn to_output(&self) -> String {
        self.to_string()
    }
}

impl ToOutput for i64 {
    fn to_output(&self) -> String {
        self.to_string()
    }
}

impl ToOutput for u64 {
    fn to_output(&self) -> String {
        self.to_string()
    }
}

impl<T: ToOutput> ToOutput for Option<T> {
    fn to_output(&self) -> String {
        match self {
            Some(v) => v.to_output(),
            None => String::new(),
        }
    }
}

/// Marker trait for types that are considered "required" by default
pub trait RequiredInput: FromInput {}
impl RequiredInput for String {}
impl RequiredInput for bool {}
impl RequiredInput for i32 {}
impl RequiredInput for u32 {}
impl RequiredInput for i64 {}
impl RequiredInput for u64 {}

/// Trait for types that can be parsed from a GitHub Actions input environment variable
pub trait FromInputEnv: Sized {
    fn from_input_env(name: &str) -> Result<Self, IoError>;
}

/// Implementation for "required" inputs (non-Option types)
impl<T: RequiredInput> FromInputEnv for T {
    fn from_input_env(name: &str) -> Result<Self, IoError> {
        let env_name = format!("INPUT_{}", name.to_uppercase().replace('-', "_"));
        let value = match env::var(&env_name) {
            Ok(v) => v,
            Err(VarError::NotPresent) => return Err(IoError::MissingRequired(name.to_string())),
            Err(VarError::NotUnicode(_)) => return Err(IoError::ParseError(name.to_string())),
        };

        if value.trim().is_empty() {
            return Err(IoError::MissingRequired(name.to_string()));
        }

        T::from_input(&value)
    }
}

/// Implementation for "optional" inputs (Option<T>)
impl<T: FromInput> FromInputEnv for Option<T> {
    fn from_input_env(name: &str) -> Result<Self, IoError> {
        let env_name = format!("INPUT_{}", name.to_uppercase().replace('-', "_"));
        let value = match env::var(&env_name) {
            Ok(v) => v,
            Err(VarError::NotPresent) => return Ok(None),
            Err(VarError::NotUnicode(_)) => return Err(IoError::ParseError(name.to_string())),
        };

        if value.trim().is_empty() {
            Ok(None)
        } else {
            T::from_input(&value).map(Some)
        }
    }
}
