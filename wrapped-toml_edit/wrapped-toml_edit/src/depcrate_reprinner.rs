// Generated macro for inner (module)
macro_rules! Depcrate_reprinner {
() => {
// Module: crate::repr
// Provides: {"inner"}
// Dependencies: {}
# [cfg (not (feature = "display"))] mod inner { use super :: ValueRepr ; impl ValueRepr for String { } impl ValueRepr for i64 { } impl ValueRepr for f64 { } impl ValueRepr for bool { } impl ValueRepr for toml_datetime :: Datetime { } }
};
}
