// Generated macro for impl_109 (impl)
macro_rules! Depcrate_decoding_errorsimpl_109 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_109"}
// Dependencies: {}
impl core :: fmt :: Display for LiteralsSectionParseError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { LiteralsSectionParseError :: IllegalLiteralSectionType { got } => { write ! (f , "Illegal literalssectiontype. Is: {got}, must be in: 0, 1, 2, 3") } LiteralsSectionParseError :: GetBitsError (e) => write ! (f , "{e:?}") , LiteralsSectionParseError :: NotEnoughBytes { have , need } => { write ! (f , "Not enough byte to parse the literals section header. Have: {have}, Need: {need}" ,) } } } }
};
}
