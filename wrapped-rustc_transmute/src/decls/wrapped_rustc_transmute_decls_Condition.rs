use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A condition which must hold for safe transmutation to be possible.
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Condition<R, T> {
    /// `Src` is transmutable into `Dst`, if `src` is transmutable into `dst`.
    Transmutable { src: T, dst: T },
    /// The region `long` must outlive `short`.
    Outlives { long: R, short: R },
    /// The `ty` is immutable.
    Immutable { ty: T },
    /// `Src` is transmutable into `Dst`, if all of the enclosed requirements are met.
    IfAll(Vec<Condition<R, T>>),
    /// `Src` is transmutable into `Dst` if any of the enclosed requirements are met.
    IfAny(Vec<Condition<R, T>>),
}
