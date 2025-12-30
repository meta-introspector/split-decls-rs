// Generated macro for impl_1488 (impl)
macro_rules! Depcrate_tests_make_testdataimpl_1488 {
() => {
// Module: crate::tests::make_testdata
// Provides: {"impl_1488"}
// Dependencies: {}
impl MeasuringAllocator { thread_local ! { static ACTIVE : Cell < bool > = const { Cell :: new (false) } ; static TOTAL_ALLOCATED : Cell < u64 > = const { Cell :: new (0) } ; static TOTAL_DEALLOCATED : Cell < u64 > = const { Cell :: new (0) } ; } pub fn start_measure () { Self :: ACTIVE . with (| c | c . set (true)) ; } pub fn end_measure () -> (u64 , u64) { Self :: ACTIVE . with (| c | c . set (false)) ; (Self :: TOTAL_ALLOCATED . with (| c | c . take ()) , Self :: TOTAL_DEALLOCATED . with (| c | c . take ()) ,) } }
};
}
