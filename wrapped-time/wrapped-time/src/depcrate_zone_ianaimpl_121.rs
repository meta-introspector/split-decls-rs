// Generated macro for impl_121 (impl)
macro_rules! Depcrate_zone_ianaimpl_121 {
() => {
// Module: crate::zone::iana
// Provides: {"impl_121"}
// Dependencies: {}
impl IanaParserExtended < IanaParser > { # [doc = " Creates a new [`IanaParserExtended`] using compiled data."] # [doc = ""] # [doc = " See [`IanaParserExtended`] for an example."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] # [expect (clippy :: new_ret_no_self)] pub fn new () -> IanaParserExtendedBorrowed < 'static > { IanaParserExtendedBorrowed :: new () } icu_provider :: gen_buffer_data_constructors ! (() -> error : DataError , functions : [new : skip , try_new_with_buffer_provider , try_new_unstable , Self ,]) ; # [doc = icu_provider :: gen_buffer_unstable_docs ! (UNSTABLE , Self :: new)] pub fn try_new_unstable < P > (provider : & P) -> Result < Self , DataError > where P : DataProvider < TimezoneIdentifiersIanaCoreV1 > + DataProvider < TimezoneIdentifiersIanaExtendedV1 > + ? Sized , { let parser = IanaParser :: try_new_unstable (provider) ? ; Self :: try_new_with_parser_unstable (provider , parser) } }
};
}
