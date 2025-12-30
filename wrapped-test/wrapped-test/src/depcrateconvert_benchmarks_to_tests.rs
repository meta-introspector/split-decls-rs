// Generated macro for convert_benchmarks_to_tests (function)
macro_rules! Depcrateconvert_benchmarks_to_tests {
() => {
// Module: crate
// Provides: {"convert_benchmarks_to_tests"}
// Dependencies: {}
pub fn convert_benchmarks_to_tests (tests : Vec < TestDescAndFn >) -> Vec < TestDescAndFn > { tests . into_iter () . map (| x | { let testfn = match x . testfn { DynBenchFn (benchfn) => DynBenchAsTestFn (benchfn) , StaticBenchFn (benchfn) => StaticBenchAsTestFn (benchfn) , f => f , } ; TestDescAndFn { desc : x . desc , testfn } }) . collect () }
};
}
