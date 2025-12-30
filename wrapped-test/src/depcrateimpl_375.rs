// Generated macro for impl_375 (impl)
macro_rules! Depcrateimpl_375 {
() => {
// Module: crate
// Provides: {"impl_375"}
// Dependencies: {}
impl FilteredTests { fn add_bench (& mut self , desc : TestDesc , testfn : TestFn) { let test = TestDescAndFn { desc , testfn } ; self . benches . push ((TestId (self . next_id) , test)) ; self . next_id += 1 ; } fn add_test (& mut self , desc : TestDesc , testfn : TestFn) { let test = TestDescAndFn { desc , testfn } ; self . tests . push ((TestId (self . next_id) , test)) ; self . next_id += 1 ; } fn total_len (& self) -> usize { self . tests . len () + self . benches . len () } }
};
}
