// Generated macro for impl_1287 (impl)
macro_rules! Depcrate_transformsimpl_1287 {
() => {
// Module: crate::transforms
// Provides: {"impl_1287"}
// Dependencies: {}
impl DataProvider < TransliteratorRulesV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < TransliteratorRulesV1 > , DataError > { self . check_req :: < TransliteratorRulesV1 > (req) ? ; self . cldr () ? . transforms () ? . lock () . expect ("poison") . as_provider_unstable (self , self , self) ? . load (req) } }
};
}
