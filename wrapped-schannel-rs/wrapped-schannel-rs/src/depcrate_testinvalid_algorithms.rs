// Generated macro for invalid_algorithms (function)
macro_rules! Depcrate_testinvalid_algorithms {
() => {
// Module: crate::test
// Provides: {"invalid_algorithms"}
// Dependencies: {}
# [test] fn invalid_algorithms () { let creds = SchannelCred :: builder () . supported_algorithms (& [Algorithm :: Rc2 , Algorithm :: Ecdsa]) . acquire (Direction :: Outbound) ; assert_eq ! (creds . err () . unwrap () . raw_os_error () . unwrap () , Foundation :: SEC_E_ALGORITHM_MISMATCH as i32) ; }
};
}
