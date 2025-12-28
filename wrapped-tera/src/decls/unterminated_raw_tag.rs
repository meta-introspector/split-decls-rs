macro_rules! unterminated_raw_tag {
    () => {
        # [test] fn unterminated_raw_tag () { assert_err_msg (r#"{% raw %}sd"# , & ["1:12" , "expected tag"]) ; }
    };
}

unterminated_raw_tag!()