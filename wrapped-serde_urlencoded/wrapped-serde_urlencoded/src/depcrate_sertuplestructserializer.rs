// Generated macro for TupleStructSerializer (struct)
macro_rules! Depcrate_serTupleStructSerializer {
() => {
// Module: crate::ser
// Provides: {"TupleStructSerializer"}
// Dependencies: {}
# [doc = " Tuple struct serializer."] # [doc = ""] # [doc = " Never instantiated, tuple structs are not supported."] pub struct TupleStructSerializer < 'input , 'output , T : UrlEncodedTarget > { inner : ser :: Impossible < & 'output mut UrlEncodedSerializer < 'input , T > , Error > , }
};
}
