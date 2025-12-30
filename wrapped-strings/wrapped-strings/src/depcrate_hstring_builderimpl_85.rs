// Generated macro for impl_85 (impl)
macro_rules! Depcrate_hstring_builderimpl_85 {
() => {
// Module: crate::hstring_builder
// Provides: {"impl_85"}
// Dependencies: {}
impl core :: fmt :: Debug for HStringBuilder { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "\"{}\"" , Decode (|| core :: char :: decode_utf16 (self . iter () . cloned ()))) } }
};
}
