macro_rules! invalid_import_macros_missing_namespace {
    () => {
        # [test] fn invalid_import_macros_missing_namespace () { assert_err_msg (r#"{% import "hello" as %}"# , & ["1:22" , "expected an identifier (must start with a-z)"] ,) ; }
    };
}

invalid_import_macros_missing_namespace!()