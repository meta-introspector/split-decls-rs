macro_rules! unterminated_filter_section {
    () => {
        # [test] fn unterminated_filter_section () { assert_err_msg (r#"{% filter uppercase %}sd"# , & ["1:25" , r#"expected tag or the filter section content"#] ,) ; }
    };
}

unterminated_filter_section!();