macro_rules! validation_tests {
    () => {
        # [test] fn validation_tests () { dir_tests (& test_data_dir () , & ["parser/validation"] , "rast" , | text , path | { let parse = SourceFile :: parse (text , Edition :: CURRENT) ; let errors = parse . errors () ; assert_errors_are_present (& errors , path) ; parse . debug_dump () }) ; }
    };
}

validation_tests!();