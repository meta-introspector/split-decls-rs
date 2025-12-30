// Generated macro for StructVariantSerializer (struct)
macro_rules! Depcrate_serStructVariantSerializer {
() => {
// Module: crate::ser
// Provides: {"StructVariantSerializer"}
// Dependencies: {}
# [doc = " Struct variant serializer."] # [doc = ""] # [doc = " Never instantiated, struct variants are not supported."] pub struct StructVariantSerializer < 'input , 'output , T : UrlEncodedTarget > { inner : ser :: Impossible < & 'output mut UrlEncodedSerializer < 'input , T > , Error > , }
};
}
