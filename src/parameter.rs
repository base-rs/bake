// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

use crate::Result;
use crate::arguments::parse;
use std::fmt::Display;
use std::str::FromStr;

/// A task parameter. Names use underscores; command-line flags also accept hyphens.
pub struct Parameter {
    pub(crate) name: String,
    pub(crate) type_name: String,
    pub(crate) positional: bool,
    pub(crate) required: bool,
    pub(crate) repeated: bool,
    pub(crate) default: Option<String>,
    pub(crate) description: String,
    pub(crate) validate: fn(&str) -> Result<()>,
}

impl Parameter {
    /// Values use `FromStr`; custom types can implement it too.
    pub fn new<Value: FromStr>(name: &str) -> Self
    where
        Value::Err: Display,
    {
        Self {
            name: name.into(),
            type_name: type_name::<Value>(),
            positional: true,
            required: true,
            repeated: false,
            default: None,
            description: String::new(),
            validate: |value| parse::<Value>(value).map(|_| ()),
        }
    }
    pub fn named(mut self) -> Self {
        self.positional = false;
        self
    }
    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }
    pub fn repeated(mut self) -> Self {
        self.repeated = true;
        self.positional = false;
        self.required = false;
        self
    }
    /// Consume repeated bare values until the task chain separator.
    pub fn variadic(mut self) -> Self {
        self.repeated = true;
        self.positional = true;
        self.required = false;
        self
    }
    pub fn default(mut self, value: impl Into<String>) -> Self {
        self.default = Some(value.into());
        self.required = false;
        self.positional = false;
        self
    }
    pub fn help(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// A type's name without module paths, such as `Option<String>` for `core::option::Option<alloc::string::String>`, for help and error text.
pub(crate) fn type_name<Value>() -> String {
    short_type_name(std::any::type_name::<Value>())
}

fn short_type_name(name: &str) -> String {
    let mut short = String::with_capacity(name.len());
    let mut path = String::new();
    for character in name.chars().chain(std::iter::once('\0')) {
        if character.is_alphanumeric() || matches!(character, '_' | ':') {
            path.push(character);
            continue;
        }
        short.push_str(path.rsplit("::").next().unwrap_or_default());
        path.clear();
        if character != '\0' {
            short.push(character);
        }
    }
    short
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_names_omit_module_paths() {
        assert_eq!(type_name::<String>(), "String");
        assert_eq!(type_name::<std::path::PathBuf>(), "PathBuf");
        assert_eq!(type_name::<Option<Vec<String>>>(), "Option<Vec<String>>");
        assert_eq!(type_name::<(u32, &str)>(), "(u32, &str)");
        assert_eq!(short_type_name(""), "");
    }
}
