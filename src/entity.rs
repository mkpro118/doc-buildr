//! # Entity Module
//!
//! This module defines the core entities used in doc-buildr to represent
//! various elements of code structure and documentation.

/// Represents a function or method parameter.
#[derive(Debug)]
pub struct Param {
    /// The name of the parameter.
    pub name: String,

    /// The name of the parameter.
    pub description: String,
}

/// Represents the return value of a function or method.
#[derive(Debug)]
pub struct Return {
    /// The description of the return value.
    pub description: String,
}

/// Represents a documentation comment.
#[derive(Debug)]
pub struct DocComment {
    /// The main body of the comment.
    pub comment: String,

    /// A list of documented parameters.
    pub params: Vec<Param>,

    /// An optional description of the return value.
    pub retval: Option<Return>,
}

/// Represents a struct definition.
#[derive(Debug)]
pub struct Struct {
    /// The name of the struct.
    pub name: String,

    /// A list of the struct's members.
    pub members: Vec<String>,
}

/// Represents a function definition.
#[derive(Debug)]
pub struct Function {
    /// The name of the function.
    pub name: String,
    /// The return type of the function.
    pub return_type: String,
    /// A list of the function's parameters.
    pub params: Vec<String>,
}

/// Represents an enum definition.
#[derive(Debug)]
pub struct Enum {
    /// The name of the enum.
    pub name: String,

    /// A list of the enum's variants.
    pub variants: Vec<String>,
}
