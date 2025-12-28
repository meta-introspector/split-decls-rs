macro_rules! socklen_t {
    () => {
        # [doc = " The integer type used with `getsockname` on this platform."] # [allow (non_camel_case_types)] pub type socklen_t = crate :: sys :: socklen_t ;
    };
}

socklen_t!();