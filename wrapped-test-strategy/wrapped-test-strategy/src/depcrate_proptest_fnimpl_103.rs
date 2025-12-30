// Generated macro for impl_103 (impl)
macro_rules! Depcrate_proptest_fnimpl_103 {
() => {
// Module: crate::proptest_fn
// Provides: {"impl_103"}
// Dependencies: {}
impl TestFnAttrArgs { fn from (args : Args) -> Result < (Self , Args) > { let mut config_args = Args :: new () ; let mut this = TestFnAttrArgs { r#async : None , dump : false , } ; for arg in args { if let Arg :: NameValue { name , value , .. } = & arg { if name == "async" { this . r#async = Some (parse2 (value . to_token_stream ()) ?) ; continue ; } } if let Arg :: Value (value) = & arg { if value == & parse_quote ! (dump) { this . dump = true ; continue ; } } config_args . 0 . push (arg) ; } Ok ((this , config_args)) } }
};
}
