use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An argument to an interface method
struct InterfaceMethodArg {
    /// The type of the argument
    pub ty: Box<syn::Type>,
    /// The name of the argument
    pub pat: Box<syn::Pat>,
}
