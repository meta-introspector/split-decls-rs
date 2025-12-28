macro_rules! parser_fuzz_tests {
    () => {
        # [test] fn parser_fuzz_tests () { for (_ , text) in collect_rust_files (& test_data_dir () , & ["parser/fuzz-failures"]) { fuzz :: check_parser (& text) } }
    };
}

parser_fuzz_tests!()