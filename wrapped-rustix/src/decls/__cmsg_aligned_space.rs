macro_rules! __cmsg_aligned_space {
    () => {
        # [doc = " Helper function for [`cmsg_aligned_space`]."] # [doc (hidden)] pub const fn __cmsg_aligned_space (len : usize) -> usize { let converted_len = len as u32 ; if converted_len as usize != len { unreachable ! () ; } unsafe { c :: CMSG_SPACE (converted_len) as usize } }
    };
}

__cmsg_aligned_space!()