// Generated macro for test_abi_eq (function)
macro_rules! Depcrate_abi_testtest_abi_eq {
() => {
// Module: crate::abi_test
// Provides: {"test_abi_eq"}
// Dependencies: {}
fn test_abi_eq < 'tcx > (abi1 : & 'tcx FnAbi < 'tcx , Ty < 'tcx > > , abi2 : & 'tcx FnAbi < 'tcx , Ty < 'tcx > >) -> bool { if abi1 . conv != abi2 . conv || abi1 . args . len () != abi2 . args . len () || abi1 . c_variadic != abi2 . c_variadic || abi1 . fixed_count != abi2 . fixed_count || abi1 . can_unwind != abi2 . can_unwind { return false ; } abi1 . ret . eq_abi (& abi2 . ret) && abi1 . args . iter () . zip (abi2 . args . iter ()) . all (| (arg1 , arg2) | arg1 . eq_abi (arg2)) }
};
}
