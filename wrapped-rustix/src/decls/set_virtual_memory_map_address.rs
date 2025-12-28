macro_rules! deps {
    () => {
        Result!();
        VirtualMemoryMapAddress!();
    };
}

macro_rules! set_virtual_memory_map_address {
    () => {
        deps!();
        # [doc = " Modify certain kernel memory map descriptor addresses of the calling"] # [doc = " process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_MM,…)`]"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Please ensure the conditions necessary to safely call this function, as"] # [doc = " detailed in the references above."] # [doc = ""] # [doc = " [`prctl(PR_SET_MM,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_SET_MM")] pub unsafe fn set_virtual_memory_map_address (option : VirtualMemoryMapAddress , address : Option < NonNull < c_void > > ,) -> io :: Result < () > { let address = address . map_or_else (null_mut , NonNull :: as_ptr) ; prctl_3args (PR_SET_MM , option as usize as * mut _ , address) . map (| _r | ()) }
    };
}

set_virtual_memory_map_address!();