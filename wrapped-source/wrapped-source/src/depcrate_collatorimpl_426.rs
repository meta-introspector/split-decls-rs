// Generated macro for impl_426 (impl)
macro_rules! Depcrate_collatorimpl_426 {
() => {
// Module: crate::collator
// Provides: {"impl_426"}
// Dependencies: {}
impl DataProvider < CollationRootV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < CollationRootV1 > , DataError > { self . check_req :: < CollationRootV1 > (req) ? ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (self . load_toml :: < collator_serde :: CollationData > (Default :: default () , "_data") . map_err (| e | e . with_req (CollationRootV1 :: INFO , req)) ? . try_into () ? ,) , }) } }
};
}
