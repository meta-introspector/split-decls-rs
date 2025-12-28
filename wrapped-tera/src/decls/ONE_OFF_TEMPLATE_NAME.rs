macro_rules! ONE_OFF_TEMPLATE_NAME {
    () => {
        # [doc = " Default template name used for `Tera::render_str` and `Tera::one_off`."] const ONE_OFF_TEMPLATE_NAME : & str = "__tera_one_off" ;
    };
}

ONE_OFF_TEMPLATE_NAME!()