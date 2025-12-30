// Generated macro for impl_319 (impl)
macro_rules! Depcrate_de_parser_devalueimpl_319 {
() => {
// Module: crate::de::parser::devalue
// Provides: {"impl_319"}
// Dependencies: {}
impl < I > ops :: Index < I > for DeValue < '_ > where I : Index , { type Output = Spanned < Self > ; fn index (& self , index : I) -> & Spanned < Self > { self . get (index) . expect ("index not found") } }
};
}
