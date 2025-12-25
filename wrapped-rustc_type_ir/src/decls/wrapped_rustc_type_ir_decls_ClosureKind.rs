use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Represents the various closure traits in the language. This
/// will determine the type of the environment (`self`, in the
/// desugaring) argument that the closure expects.
///
/// You can get the environment type of a closure using
/// `tcx.closure_env_ty()`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "nightly", derive(Encodable, Decodable, HashStable_NoContext))]
pub enum ClosureKind {
    Fn,
    FnMut,
    FnOnce,
}
