// Generated macro for test_check_req (function)
macro_rules! Depcratetest_check_req {
() => {
// Module: crate
// Provides: {"test_check_req"}
// Dependencies: {}
# [test] fn test_check_req () { use icu :: locale :: langid ; use icu_provider :: hello_world :: * ; # [allow (non_local_definitions)] impl DataProvider < HelloWorldV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < HelloWorldV1 > , DataError > { HelloWorldProvider . load (req) } } # [allow (non_local_definitions)] impl crate :: IterableDataProviderCached < HelloWorldV1 > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow < 'static > > , DataError > { Ok (HelloWorldProvider . iter_ids () ? . into_iter () . collect ()) } } let provider = SourceDataProvider :: new_testing () ; assert ! (provider . check_req ::< HelloWorldV1 > (DataRequest { id : DataIdentifierBorrowed :: for_locale (& langid ! ("fi") . into ()) , .. Default :: default () }) . is_ok ()) ; assert ! (provider . check_req ::< HelloWorldV1 > (DataRequest { id : DataIdentifierBorrowed :: for_locale (& langid ! ("arc") . into ()) , .. Default :: default () }) . is_err ()) ; }
};
}
