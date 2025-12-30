// Generated macro for impl_112 (impl)
macro_rules! Depcrate_zone_ianaimpl_112 {
() => {
// Module: crate::zone::iana
// Provides: {"impl_112"}
// Dependencies: {}
impl IanaParser { # [doc = " Creates a new [`IanaParser`] using compiled data."] # [doc = ""] # [doc = " See [`IanaParser`] for an example."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] # [expect (clippy :: new_ret_no_self)] pub fn new () -> IanaParserBorrowed < 'static > { IanaParserBorrowed :: new () } icu_provider :: gen_buffer_data_constructors ! (() -> error : DataError , functions : [new : skip , try_new_with_buffer_provider , try_new_unstable , Self ,]) ; # [doc = icu_provider :: gen_buffer_unstable_docs ! (UNSTABLE , Self :: new)] pub fn try_new_unstable < P > (provider : & P) -> Result < Self , DataError > where P : DataProvider < TimezoneIdentifiersIanaCoreV1 > + ? Sized , { let response = provider . load (Default :: default ()) ? ; Ok (Self { data : response . payload , checksum : response . metadata . checksum . ok_or_else (| | { DataError :: custom ("Missing checksum") . with_req (TimezoneIdentifiersIanaCoreV1 :: INFO , Default :: default ()) }) ? , }) } # [doc = " Returns a borrowed version of the parser that can be queried."] # [doc = ""] # [doc = " This avoids a small potential indirection cost when querying the parser."] pub fn as_borrowed (& self) -> IanaParserBorrowed < '_ > { IanaParserBorrowed { data : self . data . get () , checksum : self . checksum , } } }
};
}
