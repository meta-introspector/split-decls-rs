macro_rules! deps {
    () => {
        DeflateStream!();
        ReturnCode!();
    };
}

macro_rules! set_header {
    () => {
        deps!();
        # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must guarantee:"] # [doc = ""] # [doc = " * If `head` is `Some`"] # [doc = "     - `head.extra` is `NULL` or is readable for at least `head.extra_len` bytes"] # [doc = "     - `head.name` is `NULL` or satisfies the requirements of [`core::ffi::CStr::from_ptr`]"] # [doc = "     - `head.comment` is `NULL` or satisfies the requirements of [`core::ffi::CStr::from_ptr`]"] pub unsafe fn set_header < 'a > (stream : & mut DeflateStream < 'a > , head : Option < & 'a mut gz_header > ,) -> ReturnCode { if stream . state . wrap != 2 { ReturnCode :: StreamError as _ } else { stream . state . gzhead = head ; ReturnCode :: Ok as _ } }
    };
}

set_header!()