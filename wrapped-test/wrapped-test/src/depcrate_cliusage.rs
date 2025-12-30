// Generated macro for usage (function)
macro_rules! Depcrate_cliusage {
() => {
// Module: crate::cli
// Provides: {"usage"}
// Dependencies: {}
fn usage (binary : & str , options : & getopts :: Options) { let message = format ! ("Usage: {binary} [OPTIONS] [FILTERS...]") ; println ! (r#"{usage}

The FILTER string is tested against the name of all tests, and only those
tests whose names contain the filter are run. Multiple filter strings may
be passed, which will run all tests matching any of the filters.

By default, all tests are run in parallel. This can be altered with the
--test-threads flag when running tests (set it to 1).

By default, the tests are run in alphabetical order. Use --shuffle to run
the tests in random order. Pass the generated "shuffle seed" to
--shuffle-seed to run the tests in the same order again. Note that
--shuffle and --shuffle-seed do not affect whether the tests are run in
parallel.

All tests have their standard output and standard error captured by default.
This can be overridden with the --no-capture flag to a value other than "0".
Logging is not captured by default.

Test Attributes:

    `#[test]`        - Indicates a function is a test to be run. This function
                       takes no arguments.
    `#[bench]`       - Indicates a function is a benchmark to be run. This
                       function takes one argument (test::Bencher).
    `#[should_panic]` - This function (also labeled with `#[test]`) will only pass if
                        the code causes a panic (an assertion failure or panic!)
                        A message may be provided, which the failure string must
                        contain: #[should_panic(expected = "foo")].
    `#[ignore]`       - When applied to a function which is already attributed as a
                        test, then the test runner will ignore these tests during
                        normal test runs. Running with --ignored or --include-ignored will run
                        these tests."# , usage = options . usage (& message)) ; }
};
}
