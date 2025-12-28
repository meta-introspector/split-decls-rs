macro_rules! _GRAMMAR {
    () => {
        const _GRAMMAR : & str = include_str ! ("tera.pest") ;
    };
}

_GRAMMAR!()