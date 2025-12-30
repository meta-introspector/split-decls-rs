// Generated macro for optgroups (function)
macro_rules! Depcrate_clioptgroups {
() => {
// Module: crate::cli
// Provides: {"optgroups"}
// Dependencies: {}
fn optgroups () -> getopts :: Options { let mut opts = getopts :: Options :: new () ; opts . optflag ("" , "include-ignored" , "Run ignored and not ignored tests") . optflag ("" , "ignored" , "Run only ignored tests") . optflag ("" , "force-run-in-process" , "Forces tests to run in-process when panic=abort") . optflag ("" , "exclude-should-panic" , "Excludes tests marked as should_panic") . optflag ("" , "test" , "Run tests and not benchmarks") . optflag ("" , "bench" , "Run benchmarks instead of tests") . optflag ("" , "list" , "List all tests and benchmarks") . optflag ("h" , "help" , "Display this message") . optopt ("" , "logfile" , "Write logs to the specified file (deprecated)" , "PATH") . optflag ("" , "no-capture" , "don't capture stdout/stderr of each \
             task, allow printing directly" ,) . optopt ("" , "test-threads" , "Number of threads used for running tests \
             in parallel" , "n_threads" ,) . optmulti ("" , "skip" , "Skip tests whose names contain FILTER (this flag can \
             be used multiple times)" , "FILTER" ,) . optflag ("q" , "quiet" , "Display one character per test instead of one line. \
             Alias to --format=terse" ,) . optflag ("" , "exact" , "Exactly match filters rather than by substring") . optopt ("" , "color" , "Configure coloring of output:
            auto   = colorize if stdout is a tty and tests are run on serially (default);
            always = always colorize output;
            never  = never colorize output;" , "auto|always|never" ,) . optopt ("" , "format" , "Configure formatting of output:
            pretty = Print verbose output;
            terse  = Display one character per test;
            json   = Output a json document;
            junit  = Output a JUnit document" , "pretty|terse|json|junit" ,) . optflag ("" , "show-output" , "Show captured stdout of successful tests") . optopt ("Z" , "" , "Enable nightly-only flags:
            unstable-options = Allow use of experimental features" , "unstable-options" ,) . optflag ("" , "report-time" , "Show execution time of each test.

            Threshold values for colorized output can be configured via
            `RUST_TEST_TIME_UNIT`, `RUST_TEST_TIME_INTEGRATION` and
            `RUST_TEST_TIME_DOCTEST` environment variables.

            Expected format of environment variable is `VARIABLE=WARN_TIME,CRITICAL_TIME`.
            Durations must be specified in milliseconds, e.g. `500,2000` means that the warn time
            is 0.5 seconds, and the critical time is 2 seconds.

            Not available for --format=terse" ,) . optflag ("" , "ensure-time" , "Treat excess of the test execution time limit as error.

            Threshold values for this option can be configured via
            `RUST_TEST_TIME_UNIT`, `RUST_TEST_TIME_INTEGRATION` and
            `RUST_TEST_TIME_DOCTEST` environment variables.

            Expected format of environment variable is `VARIABLE=WARN_TIME,CRITICAL_TIME`.

            `CRITICAL_TIME` here means the limit that should not be exceeded by test.
            " ,) . optflag ("" , "shuffle" , "Run tests in random order") . optopt ("" , "shuffle-seed" , "Run tests in random order; seed the random number generator with SEED" , "SEED" ,) ; opts }
};
}
