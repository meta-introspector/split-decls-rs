macro_rules! deps {
    () => {
        CheckReparse!();
    };
}

macro_rules! reparse_fuzz_tests {
    () => {
        deps!();
        # [test] fn reparse_fuzz_tests () { for (_ , text) in collect_rust_files (& test_data_dir () , & ["reparse/fuzz-failures"]) { let check = fuzz :: CheckReparse :: from_data (text . as_bytes ()) . unwrap () ; check . run () ; } }
    };
}

reparse_fuzz_tests!();