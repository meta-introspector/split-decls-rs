// Generated macro for parse_zlint_output (function)
macro_rules! Depcrate_zlintparse_zlint_output {
() => {
// Module: crate::zlint
// Provides: {"parse_zlint_output"}
// Dependencies: {}
# [test] fn parse_zlint_output () { let demo_output = br#"
          {
            "e_algorithm_identifier_improper_encoding": {"result": "pass"},
            "e_basic_constraints_not_critical": {"result": "NA", "details": "foo"}
          }
        "# ; let output : LintResult = serde_json :: from_slice (demo_output) . expect ("parse output") ; assert_eq ! (output . 0 . get ("e_algorithm_identifier_improper_encoding") , Some (& LintStatus { status : Status :: Pass , details : None })) ; }
};
}
