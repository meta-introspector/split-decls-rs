// Generated macro for impl_78 (impl)
macro_rules! Depcrateimpl_78 {
() => {
// Module: crate
// Provides: {"impl_78"}
// Dependencies: {}
impl TestInfo { # [doc = " Check if either the name or short name is a match, for filtering."] fn matches (& self , pat : & str) -> bool { self . short_name . contains (pat) || self . name . contains (pat) } # [doc = " Create a `TestInfo` for a given float and generator, then add it to a list."] fn register < F : Float , G : Generator < F > > (v : & mut Vec < Self >) { let f_name = type_name :: < F > () ; let gen_name = G :: NAME ; let gen_short_name = G :: SHORT_NAME ; let name = format ! ("{f_name} {gen_name}") ; let short_name = format ! ("{f_name} {gen_short_name}") ; let short_name_padded = format ! ("{short_name:18}") ; let info = TestInfo { float_name : f_name , float_bits : F :: BITS , gen_name , progress : None , name , short_name_padded , short_name , launch : test_runner :: < F , G > , total_tests : G :: total_tests () , completed : OnceLock :: new () , } ; v . push (info) ; } # [doc = " True if this should be run after all others."] fn is_huge_test (& self) -> bool { self . total_tests >= HUGE_TEST_CUTOFF } # [doc = " When the test is finished, update progress bar messages and finalize."] fn complete (& self , c : Completed) { self . progress . as_ref () . unwrap () . complete (& c , 0) ; self . completed . set (c) . unwrap () ; } }
};
}
