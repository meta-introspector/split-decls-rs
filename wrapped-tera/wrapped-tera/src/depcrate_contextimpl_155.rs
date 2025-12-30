// Generated macro for impl_155 (impl)
macro_rules! Depcrate_contextimpl_155 {
() => {
// Module: crate::context
// Provides: {"impl_155"}
// Dependencies: {}
impl ValueTruthy for Value { fn is_truthy (& self) -> bool { match * self { Value :: Number (ref i) => { if i . is_i64 () { return i . as_i64 () . unwrap () != 0 ; } if i . is_u64 () { return i . as_u64 () . unwrap () != 0 ; } let f = i . as_f64 () . unwrap () ; f != 0.0 && ! f . is_nan () } Value :: Bool (ref i) => * i , Value :: Null => false , Value :: String (ref i) => ! i . is_empty () , Value :: Array (ref i) => ! i . is_empty () , Value :: Object (ref i) => ! i . is_empty () , } } }
};
}
