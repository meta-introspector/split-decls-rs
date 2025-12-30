// Generated macro for TASKS (const)
macro_rules! DepcrateTASKS {
() => {
// Module: crate
// Provides: {"TASKS"}
// Dependencies: {}
const TASKS : & [Task] = & [("ci" , cmd_ci , "runs everything in CI") , ("check" , cmd_check , "checks everything") , ("build" , cmd_build , "builds everything with dev profile") , ("build-release" , cmd_build_release , "builds everything with release profile" ,) , ("test" , cmd_test , "tests everything with dev profile") , ("test-release" , cmd_test_release , "tests everything with release profile" ,) , ("test-update" , cmd_test_update , "regenerates the expected test output" ,) , ("features" , cmd_features , "tests with various feature combinations" ,) , ("cross" , cmd_cross , "tests for other platforms") , ("msrv" , cmd_msrv , "tests minimum supported Rust version") , ("fmt" , cmd_fmt , "checks formatting") , ("doc" , cmd_doc , "generates documentation for everything") , ("coverage" , cmd_coverage , "generates HTML test coverage with tarpaulin and pycobertura, and opens it" ,) , ("coverage_lcov" , cmd_coverage_lcov , "generates Lcov test coverage with tarpaulin" ,) , ("clippy" , cmd_clippy , "run clippy for everything") , ("semver" , cmd_semver , "run semver checks") ,] ;
};
}
