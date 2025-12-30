// Generated macro for impl_546 (impl)
macro_rules! Depcrate_datetime_legacyimpl_546 {
() => {
// Module: crate::datetime::legacy
// Provides: {"impl_546"}
// Dependencies: {}
impl ca :: Contexts < cldr_serde :: ca :: MonthSymbols > { fn get (& self , ctx : & (& 'static [TinyStr4] , & str)) -> months :: Contexts < 'static > { months :: Contexts { format : self . format . get (ctx) , stand_alone : self . stand_alone . as_ref () . and_then (| stand_alone | { let abbreviated = stand_alone . abbreviated . as_ref () . and_then (| v | v . get_unaliased (& self . format . abbreviated)) ; let narrow = stand_alone . narrow . as_ref () . and_then (| v | v . get_unaliased (& self . format . narrow)) ; let short = if stand_alone . short == self . format . short { None } else { stand_alone . short . clone () } ; let wide = stand_alone . wide . as_ref () . and_then (| v | v . get_unaliased (& self . format . wide)) ; if abbreviated . is_none () && narrow . is_none () && wide . is_none () && short . is_none () { None } else { Some (ca :: StandAloneWidths { abbreviated , narrow , short , wide , }) } }) . map (| ref stand_alone | months :: StandAloneWidths { abbreviated : stand_alone . abbreviated . as_ref () . map (| width | width . get (ctx)) , narrow : stand_alone . narrow . as_ref () . map (| width | width . get (ctx)) , short : stand_alone . short . as_ref () . map (| width | width . get (ctx)) , wide : stand_alone . wide . as_ref () . map (| width | width . get (ctx)) , }) , } } }
};
}
