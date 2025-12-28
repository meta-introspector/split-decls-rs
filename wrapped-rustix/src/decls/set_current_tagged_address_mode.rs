macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_current_tagged_address_mode {
    () => {
        deps!();
        # [doc = " Controls support for passing tagged user-space addresses to the kernel."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_TAGGED_ADDR_CTRL,…)`]"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Please ensure the conditions necessary to safely call this function, as"] # [doc = " detailed in the references above."] # [doc = ""] # [doc = " [`prctl(PR_SET_TAGGED_ADDR_CTRL,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub unsafe fn set_current_tagged_address_mode (mode : Option < TaggedAddressMode > , mte_tag : u32 ,) -> io :: Result < () > { let config = mode . as_ref () . map_or (0_u32 , TaggedAddressMode :: bits) | ((mte_tag << PR_MTE_TAG_SHIFT) & PR_MTE_TAG_MASK) ; prctl_2args (PR_SET_TAGGED_ADDR_CTRL , config as usize as * mut _) . map (| _r | ()) }
    };
}

set_current_tagged_address_mode!()