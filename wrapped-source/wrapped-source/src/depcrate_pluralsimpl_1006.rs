// Generated macro for impl_1006 (impl)
macro_rules! Depcrate_pluralsimpl_1006 {
() => {
// Module: crate::plurals
// Provides: {"impl_1006"}
// Dependencies: {}
# [cfg (feature = "experimental")] impl DataProvider < PluralsRangesV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < PluralsRangesV1 > , DataError > { self . check_req :: < PluralsRangesV1 > (req) ? ; if req . id . locale . is_unknown () { Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (PluralRanges { ranges : ZeroMap :: default () , }) , }) } else { Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (PluralRanges :: from (self . get_plural_ranges () ? . 0 . get (& icu :: locale :: LanguageIdentifier :: from ((req . id . locale . language , req . id . locale . script , req . id . locale . region ,))) . ok_or (DataErrorKind :: IdentifierNotFound . into_error ()) ? ,)) , }) } } }
};
}
