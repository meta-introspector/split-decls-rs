macro_rules! unterminated_if_tag {
    () => {
        # [test] fn unterminated_if_tag () { assert_err_msg (r#"{% if true %}sd"# , & ["1:16" , r#"expected tag or some content"#]) ; }
    };
}

unterminated_if_tag!()