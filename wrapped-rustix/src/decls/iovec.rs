macro_rules! iovec {
    () => {
        # [allow (missing_docs)] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct iovec { pub iov_base : * mut c_void , pub iov_len : usize , }
    };
}

iovec!();