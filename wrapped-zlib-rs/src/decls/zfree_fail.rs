macro_rules! zfree_fail {
    () => {
        # [cfg (test)] unsafe extern "C" fn zfree_fail (_ : * mut c_void , _ : * mut c_void) { }
    };
}

zfree_fail!();