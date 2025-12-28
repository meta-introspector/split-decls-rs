macro_rules! MAGICAL_DUMP_VAR {
    () => {
        # [doc = " Special string indicating request to dump context"] static MAGICAL_DUMP_VAR : & str = "__tera_context" ;
    };
}

MAGICAL_DUMP_VAR!();