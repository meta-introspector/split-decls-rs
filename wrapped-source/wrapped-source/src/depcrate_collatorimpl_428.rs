// Generated macro for impl_428 (impl)
macro_rules! Depcrate_collatorimpl_428 {
() => {
// Module: crate::collator
// Provides: {"impl_428"}
// Dependencies: {}
impl DataProvider < CollationTailoringV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < CollationTailoringV1 > , DataError > { self . check_req :: < CollationTailoringV1 > (req) ? ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (self . load_toml :: < collator_serde :: CollationData > (req . id , "_data") . and_then (TryInto :: try_into) . map_err (| e | e . with_req (< CollationTailoringV1 > :: INFO , req)) ? ,) , }) } }
};
}
