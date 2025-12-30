// Generated macro for dayperiods_convert (function)
macro_rules! Depcrate_datetime_neodayperiods_convert {
() => {
// Module: crate::datetime::neo
// Provides: {"dayperiods_convert"}
// Dependencies: {}
fn dayperiods_convert (_datagen : & SourceDataProvider , _locale : & DataLocale , data : & ca :: Dates , _calendar : DatagenCalendar , context : Context , length : Length ,) -> Result < LinearNames < 'static > , DataError > { let day_periods = data . day_periods . get_symbols (context , length) ; let mut periods = vec ! [&* day_periods . am , &* day_periods . pm] ; if let Some (ref noon) = day_periods . noon { periods . push (noon) ; } else if day_periods . midnight . is_some () { periods . push ("") ; } ; if let Some (ref midnight) = day_periods . midnight { periods . push (midnight) } Ok (LinearNames { names : (& periods) . into () , }) }
};
}
