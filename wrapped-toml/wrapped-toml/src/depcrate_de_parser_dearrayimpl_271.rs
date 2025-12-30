// Generated macro for impl_271 (impl)
macro_rules! Depcrate_de_parser_dearrayimpl_271 {
() => {
// Module: crate::de::parser::dearray
// Provides: {"impl_271"}
// Dependencies: {}
impl < 'i , I : core :: slice :: SliceIndex < [Spanned < DeValue < 'i > >] > > core :: ops :: Index < I > for DeArray < 'i > { type Output = I :: Output ; # [inline] fn index (& self , index : I) -> & Self :: Output { self . items . index (index) } }
};
}
