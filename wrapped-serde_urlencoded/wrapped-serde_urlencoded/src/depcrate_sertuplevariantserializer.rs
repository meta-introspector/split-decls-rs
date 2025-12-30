// Generated macro for TupleVariantSerializer (struct)
macro_rules! Depcrate_serTupleVariantSerializer {
() => {
// Module: crate::ser
// Provides: {"TupleVariantSerializer"}
// Dependencies: {}
# [doc = " Tuple variant serializer."] # [doc = ""] # [doc = " Never instantiated, tuple variants are not supported."] pub struct TupleVariantSerializer < 'input , 'output , T : UrlEncodedTarget > { inner : ser :: Impossible < & 'output mut UrlEncodedSerializer < 'input , T > , Error > , }
};
}
