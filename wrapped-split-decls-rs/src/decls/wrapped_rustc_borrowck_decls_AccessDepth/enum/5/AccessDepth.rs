use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Copy , Clone , PartialEq , Eq , Debug)] enum AccessDepth { # [doc = " From the RFC: \"A *shallow* access means that the immediate"] # [doc = " fields reached at P are accessed, but references or pointers"] # [doc = " found within are not dereferenced. Right now, the only access"] # [doc = " that is shallow is an assignment like `x = ...;`, which would"] # [doc = " be a *shallow write* of `x`.\""] Shallow (Option < ArtificialField >) , # [doc = " From the RFC: \"A *deep* access means that all data reachable"] # [doc = " through the given place may be invalidated or accesses by"] # [doc = " this action.\""] Deep , # [doc = " Access is Deep only when there is a Drop implementation that"] # [doc = " can reach the data behind the reference."] Drop , }
}