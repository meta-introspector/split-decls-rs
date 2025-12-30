// Generated macro for impl_73 (impl)
macro_rules! Depcrate_expressionimpl_73 {
() => {
// Module: crate::expression
// Provides: {"impl_73"}
// Dependencies: {}
impl FromStr for Expression { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { static MACRO_RE : LazyLock < Regex > = LazyLock :: new (| | Regex :: new (r"^(?P<name>[\w\d_]+)!\((?P<ex>.*?)\);?$") . unwrap ()) ; if s == "SvUndef" { Ok (Expression :: SvUndef) } else if MACRO_RE . is_match (s) { let c = MACRO_RE . captures (s) . unwrap () ; let ex = c ["ex"] . to_string () ; let _ : TokenStream = ex . parse () . map_err (| e | format ! ("could not parse macro call expression: {e:#?}")) ? ; Ok (Expression :: MacroCall (c ["name"] . to_string () , ex)) } else { let (s , id_type) = if let Some (varname) = s . strip_prefix ('$') { (varname , IdentifierType :: Variable) } else { (s , IdentifierType :: Symbol) } ; let identifier = s . trim () . parse () ? ; Ok (Expression :: Identifier (identifier , id_type)) } } }
};
}
