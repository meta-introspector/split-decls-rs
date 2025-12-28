macro_rules! invalid_import_macros_missing_filename {
    () => {
        # [test] fn invalid_import_macros_missing_filename () { assert_err_msg ("{% import as macros %}" , & ["1:11" , "expected a string"]) ; }
    };
}

invalid_import_macros_missing_filename!()