// Generated macro for years_convert (function)
macro_rules! Depcrate_datetime_neoyears_convert {
() => {
// Module: crate::datetime::neo
// Provides: {"years_convert"}
// Dependencies: {}
fn years_convert (datagen : & SourceDataProvider , locale : & DataLocale , data : & ca :: Dates , calendar : DatagenCalendar , context : Context , length : Length ,) -> Result < YearNames < 'static > , DataError > { assert_eq ! (context , Context :: Format , "Eras and cyclic years do not participate in standalone formatting") ; if let Some (ref eras) = data . eras { eras_convert (datagen , locale , eras , calendar , length) } else if let Some (years) = data . cyclic_name_sets . as_ref () . and_then (| c | c . years . as_ref ()) { let years = years . get_symbols (context , length) ; let years : Vec < _ > = years . iter () . enumerate () . map (| (index , (key , value)) | { if * key as usize != index + 1 { panic ! ("Calendar {calendar:?} in locale {locale} missing cyclic year name for index {index}") ; } & * * value }) . collect () ; Ok (YearNames :: Cyclic ((& years) . into ())) } else { panic ! ("Calendar {calendar:?} in locale {locale} has neither eras nor cyclicNameSets for years") } }
};
}
