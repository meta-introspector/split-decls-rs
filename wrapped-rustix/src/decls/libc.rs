macro_rules! libc {
    () => {
        # [cfg (linux_raw)] mod libc { use core :: ptr ; use linux_raw_sys :: ctypes :: { c_char , c_void } ; # [cfg (all (target_os = "android" , target_pointer_width = "32"))] pub (super) const RTLD_DEFAULT : * mut c_void = - 1isize as * mut c_void ; # [cfg (not (all (target_os = "android" , target_pointer_width = "32")))] pub (super) const RTLD_DEFAULT : * mut c_void = ptr :: null_mut () ; extern "C" { pub (super) fn dlsym (handle : * mut c_void , symbol : * const c_char) -> * mut c_void ; } # [test] fn test_abi () { assert_eq ! (self :: RTLD_DEFAULT , :: libc :: RTLD_DEFAULT) ; } }
    };
}

libc!();