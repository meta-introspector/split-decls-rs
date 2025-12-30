// Generated macro for TestCases (struct)
macro_rules! Depcrate_casesTestCases {
() => {
// Module: crate::cases
// Provides: {"TestCases"}
// Dependencies: {}
# [doc = " Entry point for running tests"] # [derive (Debug , Default)] pub struct TestCases { runner : std :: cell :: RefCell < crate :: RunnerSpec > , bins : std :: cell :: RefCell < crate :: BinRegistry > , substitutions : std :: cell :: RefCell < snapbox :: Redactions > , has_run : std :: cell :: Cell < bool > , }
};
}
