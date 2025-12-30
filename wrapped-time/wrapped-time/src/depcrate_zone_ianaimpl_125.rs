// Generated macro for impl_125 (impl)
macro_rules! Depcrate_zone_ianaimpl_125 {
() => {
// Module: crate::zone::iana
// Provides: {"impl_125"}
// Dependencies: {}
impl IanaParserExtendedBorrowed < 'static > { # [doc = " Creates a new [`IanaParserExtendedBorrowed`] using compiled data."] # [doc = ""] # [doc = " See [`IanaParserExtendedBorrowed`] for an example."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] pub fn new () -> Self { const _ : () = assert ! (crate :: provider :: Baked :: SINGLETON_TIMEZONE_IDENTIFIERS_IANA_CORE_V1_CHECKSUM == crate :: provider :: Baked :: SINGLETON_TIMEZONE_IDENTIFIERS_IANA_EXTENDED_V1_CHECKSUM ,) ; Self { inner : IanaParserBorrowed :: new () , data : crate :: provider :: Baked :: SINGLETON_TIMEZONE_IDENTIFIERS_IANA_EXTENDED_V1 , } } # [doc = " Cheaply converts a [`IanaParserExtendedBorrowed<'static>`] into a [`IanaParserExtended`]."] # [doc = ""] # [doc = " Note: Due to branching and indirection, using [`IanaParserExtended`] might inhibit some"] # [doc = " compile-time optimizations that are possible with [`IanaParserExtendedBorrowed`]."] pub fn static_to_owned (& self) -> IanaParserExtended < IanaParser > { IanaParserExtended { inner : self . inner . static_to_owned () , data : DataPayload :: from_static_ref (self . data) , } } }
};
}
