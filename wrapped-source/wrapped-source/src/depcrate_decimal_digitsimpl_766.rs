// Generated macro for impl_766 (impl)
macro_rules! Depcrate_decimal_digitsimpl_766 {
() => {
// Module: crate::decimal::digits
// Provides: {"impl_766"}
// Dependencies: {}
impl DataProvider < DecimalDigitsV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < DecimalDigitsV1 > , DataError > { self . check_req :: < DecimalDigitsV1 > (req) ? ; let nsname = req . id . marker_attributes . as_str () ; if nsname . is_empty () { panic ! ("Found empty numbering system") } let result = self . get_digits_for_numbering_system (nsname) ? ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (result) , }) } }
};
}
