// Generated macro for impl_310 (impl)
macro_rules! Depcrate_de_parser_devalueimpl_310 {
() => {
// Module: crate::de::parser::devalue
// Provides: {"impl_310"}
// Dependencies: {}
impl DeInteger < '_ > { pub (crate) fn to_u64 (& self) -> Option < u64 > { u64 :: from_str_radix (self . inner . as_ref () , self . radix) . ok () } pub (crate) fn to_i64 (& self) -> Option < i64 > { i64 :: from_str_radix (self . inner . as_ref () , self . radix) . ok () } pub (crate) fn to_u128 (& self) -> Option < u128 > { u128 :: from_str_radix (self . inner . as_ref () , self . radix) . ok () } pub (crate) fn to_i128 (& self) -> Option < i128 > { i128 :: from_str_radix (self . inner . as_ref () , self . radix) . ok () } # [doc = " [`from_str_radix`][i64::from_str_radix]-compatible representation of an integer"] # [doc = ""] # [doc = " Requires [`DeInteger::radix`] to interpret"] # [doc = ""] # [doc = " See [`Display`][std::fmt::Display] for a representation that includes the radix"] pub fn as_str (& self) -> & str { self . inner . as_ref () } # [doc = " Numeric base of [`DeInteger::as_str`]"] pub fn radix (& self) -> u32 { self . radix } }
};
}
