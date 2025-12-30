// Generated macro for impl_1306 (impl)
macro_rules! Depcrate_ucaseimpl_1306 {
() => {
// Module: crate::ucase
// Provides: {"impl_1306"}
// Dependencies: {}
impl DataProvider < CaseMapUnfoldV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < CaseMapUnfoldV1 > , DataError > { self . check_req :: < CaseMapUnfoldV1 > (req) ? ; let toml = & self . icuexport () ? . read_and_parse_toml :: < ucase_serde :: Main > (& format ! ("ucase/{}/ucase.toml" , self . trie_type ())) ? . ucase ; let unfold = & toml . unfold . unfold ; let unfold = CaseMapUnfold :: try_from_icu (unfold) ? ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (unfold) , }) } }
};
}
