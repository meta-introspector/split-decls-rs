macro_rules! zalloc_fail {
    () => {
        # [cfg (test)] unsafe extern "C" fn zalloc_fail (_ : * mut c_void , _ : c_uint , _ : c_uint) -> * mut c_void { core :: ptr :: null_mut () }
    };
}

zalloc_fail!();