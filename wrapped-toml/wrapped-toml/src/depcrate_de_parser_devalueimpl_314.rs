// Generated macro for impl_314 (impl)
macro_rules! Depcrate_de_parser_devalueimpl_314 {
() => {
// Module: crate::de::parser::devalue
// Provides: {"impl_314"}
// Dependencies: {}
impl DeFloat < '_ > { pub (crate) fn to_f64 (& self) -> Option < f64 > { let f : f64 = self . inner . as_ref () . parse () . ok () ? ; if f . is_infinite () && ! self . as_str () . contains ("inf") { None } else { Some (f) } } # [doc = " [`FromStr`][std::str::FromStr]-compatible representation of a float"] pub fn as_str (& self) -> & str { self . inner . as_ref () } }
};
}
