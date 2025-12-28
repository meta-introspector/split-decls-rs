macro_rules! deps {
    () => {
        PrctlMmMap!();
        Result!();
    };
}

macro_rules! virtual_memory_map_config_struct_size {
    () => {
        deps!();
        # [doc = " Get the size of the [`PrctlMmMap`] the kernel expects."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_MM,PR_SET_MM_MAP_SIZE,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SET_MM,PR_SET_MM_MAP_SIZE,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_SET_MM")] # [doc (alias = "PR_SET_MM_MAP_SIZE")] pub fn virtual_memory_map_config_struct_size () -> io :: Result < usize > { let mut value : c_uint = 0 ; let value_ptr = as_mut_ptr (& mut value) ; unsafe { prctl_3args (PR_SET_MM , PR_SET_MM_MAP_SIZE as * mut _ , value_ptr . cast ()) ? } ; Ok (value as usize) }
    };
}

virtual_memory_map_config_struct_size!()