macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! current_tagged_address_mode {
    () => {
        deps!();
        # [doc = " Get the current tagged address mode for the calling thread."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_GET_TAGGED_ADDR_CTRL,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_GET_TAGGED_ADDR_CTRL,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub fn current_tagged_address_mode () -> io :: Result < (Option < TaggedAddressMode > , u32) > { let r = unsafe { prctl_1arg (PR_GET_TAGGED_ADDR_CTRL) ? } as c_uint ; let mode = r & 0b111_u32 ; let mte_tag = (r & PR_MTE_TAG_MASK) >> PR_MTE_TAG_SHIFT ; Ok ((TaggedAddressMode :: from_bits (mode) , mte_tag)) }
    };
}

current_tagged_address_mode!()