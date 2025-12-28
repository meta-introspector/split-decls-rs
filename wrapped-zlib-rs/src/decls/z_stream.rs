macro_rules! deps {
    () => {
        Bytef!();
    };
}

macro_rules! z_stream {
    () => {
        deps!();
        # [doc = " The current stream state"] # [doc = ""] # [doc = " # Custom allocators"] # [doc = ""] # [doc = " The low-level API supports passing in a custom allocator as part of the [`z_stream`]:"] # [doc = ""] # [doc = " ```no_check"] # [doc = " struct z_stream {"] # [doc = "     // ..."] # [doc = "     zalloc: Option<unsafe extern \"C\" fn(*mut c_void, c_uint, c_uint) -> *mut c_void>,"] # [doc = "     zfree: Option<unsafe extern \"C\" fn(*mut c_void, *mut c_void)>,"] # [doc = "     opaque: *mut c_void,"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " When these fields are `None` (or `NULL` in C), the initialization functions use a default allocator,"] # [doc = " based on feature flags:"] # [doc = ""] # [doc = " - `\"rust-allocator\"` uses the rust global allocator"] # [doc = " - `\"c-allocator\"` uses an allocator based on `malloc` and `free`"] # [doc = ""] # [doc = " When both are configured, the `\"rust-allocator\"` is preferred. When no default allocator is configured,"] # [doc = " and custom `zalloc` and `zfree` are provided, the initialization functions will return a [`Z_STREAM_ERROR`]."] # [doc = ""] # [doc = " When custom `zalloc` and `zfree` functions are given, they must adhere to the following contract"] # [doc = " to be safe:"] # [doc = ""] # [doc = " - a call `zalloc(opaque, n, m)` must return a pointer `p` to `n * m` bytes of memory, or"] # [doc = "   `NULL` if out of memory"] # [doc = " - a call `zfree(opaque, p)` must free that memory"] # [doc = ""] # [doc = " The `strm.opaque` value is passed to as the first argument to all calls to `zalloc`"] # [doc = " and `zfree`, but is otherwise ignored by the library."] # [repr (C)] # [derive (Copy , Clone)] pub struct z_stream { pub next_in : * const Bytef , pub avail_in : uInt , pub total_in : z_size , pub next_out : * mut Bytef , pub avail_out : uInt , pub total_out : z_size , pub msg : * mut c_char , pub state : * mut internal_state , pub zalloc : Option < alloc_func > , pub zfree : Option < free_func > , pub opaque : voidpf , pub data_type : c_int , pub adler : z_checksum , pub reserved : uLong , }
    };
}

z_stream!();