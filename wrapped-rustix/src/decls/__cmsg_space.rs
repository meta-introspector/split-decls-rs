macro_rules! __cmsg_space {
    () => {
        # [doc = " Helper function for [`cmsg_space`]."] # [doc (hidden)] pub const fn __cmsg_space (len : usize) -> usize { let len = len + align_of :: < c :: cmsghdr > () ; __cmsg_aligned_space (len) }
    };
}

__cmsg_space!()