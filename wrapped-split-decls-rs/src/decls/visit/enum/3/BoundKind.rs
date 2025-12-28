use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Copy , Clone , Debug)] pub enum BoundKind { # [doc = " Trait bounds in generics bounds and type/trait alias."] # [doc = " E.g., `<T: Bound>`, `type A: Bound`, or `where T: Bound`."] Bound , # [doc = " Trait bounds in `impl` type."] # [doc = " E.g., `type Foo = impl Bound1 + Bound2 + Bound3`."] Impl , # [doc = " Trait bounds in trait object type."] # [doc = " E.g., `dyn Bound1 + Bound2 + Bound3`."] TraitObject , # [doc = " Super traits of a trait."] # [doc = " E.g., `trait A: B`"] SuperTraits , }