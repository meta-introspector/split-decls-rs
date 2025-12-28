macro_rules! invalid_filter_section_missing_name {
    () => {
        # [test] fn invalid_filter_section_missing_name () { assert_err_msg (r#"{% filter %}sd{% endfilter %}"# , & ["1:11" , "expected an identifier (must start with a-z)"] ,) ; }
    };
}

invalid_filter_section_missing_name!();