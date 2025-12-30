// Generated macro for weekday_convert (function)
macro_rules! Depcrate_datetime_neoweekday_convert {
() => {
// Module: crate::datetime::neo
// Provides: {"weekday_convert"}
// Dependencies: {}
fn weekday_convert (_datagen : & SourceDataProvider , _locale : & DataLocale , data : & ca :: Dates , _calendar : DatagenCalendar , context : Context , length : Length ,) -> Result < LinearNames < 'static > , DataError > { let day_symbols = data . days . get_symbols (context , length) ; let days = [& * day_symbols . sun , & * day_symbols . mon , & * day_symbols . tue , & * day_symbols . wed , & * day_symbols . thu , & * day_symbols . fri , & * day_symbols . sat ,] ; Ok (LinearNames { names : (& days) . into () , }) }
};
}
