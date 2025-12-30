// Generated macro for impl_552 (impl)
macro_rules! Depcrate_datetime_legacyimpl_552 {
() => {
// Module: crate::datetime::legacy
// Provides: {"impl_552"}
// Dependencies: {}
impl ca :: Contexts < cldr_serde :: ca :: DaySymbols > { fn get (& self , ctx : & ()) -> weekdays :: Contexts < 'static > { weekdays :: Contexts { format : weekdays :: FormatWidths { short : self . format . short . as_ref () . map (| width | width . get (ctx)) , } , stand_alone : self . stand_alone . as_ref () . map (| stand_alone | { let abbreviated = stand_alone . abbreviated . as_ref () . and_then (| v | v . get_unaliased (& self . format . abbreviated)) ; let narrow = stand_alone . narrow . as_ref () . and_then (| v | v . get_unaliased (& self . format . narrow)) ; let short = if stand_alone . short == self . format . short { None } else { stand_alone . short . clone () } ; let wide = stand_alone . wide . as_ref () . and_then (| v | v . get_unaliased (& self . format . wide)) ; abbreviated . is_some () || narrow . is_some () || wide . is_some () || short . is_some () }) . unwrap_or (false) , } } }
};
}
