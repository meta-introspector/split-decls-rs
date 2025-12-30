// Generated macro for impl_116 (impl)
macro_rules! Depcrate_zone_ianaimpl_116 {
() => {
// Module: crate::zone::iana
// Provides: {"impl_116"}
// Dependencies: {}
impl IanaParserBorrowed < 'static > { # [doc = " Creates a new [`IanaParserBorrowed`] using compiled data."] # [doc = ""] # [doc = " See [`IanaParserBorrowed`] for an example."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] pub fn new () -> Self { Self { data : crate :: provider :: Baked :: SINGLETON_TIMEZONE_IDENTIFIERS_IANA_CORE_V1 , checksum : crate :: provider :: Baked :: SINGLETON_TIMEZONE_IDENTIFIERS_IANA_CORE_V1_CHECKSUM , } } # [doc = " Cheaply converts a [`IanaParserBorrowed<'static>`] into a [`IanaParser`]."] # [doc = ""] # [doc = " Note: Due to branching and indirection, using [`IanaParser`] might inhibit some"] # [doc = " compile-time optimizations that are possible with [`IanaParserBorrowed`]."] pub fn static_to_owned (& self) -> IanaParser { IanaParser { data : DataPayload :: from_static_ref (self . data) , checksum : self . checksum , } } }
};
}
