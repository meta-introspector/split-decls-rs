macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! configure_pointer_authentication_keys {
    () => {
        deps!();
        # [doc = " Set enabled pointer authentication keys."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_PAC_SET_ENABLED_KEYS,…)`]"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Please ensure the conditions necessary to safely call this function, as"] # [doc = " detailed in the references above."] # [doc = ""] # [doc = " [`prctl(PR_PAC_SET_ENABLED_KEYS,…)`]: https://www.kernel.org/doc/html/v6.13/arch/arm64/pointer-authentication.html"] # [inline] # [doc (alias = "PR_PAC_SET_ENABLED_KEYS")] # [cfg (linux_raw_dep)] pub unsafe fn configure_pointer_authentication_keys < Config : Iterator < Item = (PointerAuthenticationKeys , bool) > , > (config : Config ,) -> io :: Result < () > { let mut affected_keys : u32 = 0 ; let mut enabled_keys : u32 = 0 ; for (key , enable) in config { let key = key . bits () ; affected_keys |= key ; if enable { enabled_keys |= key ; } else { enabled_keys &= ! key ; } } if affected_keys == 0 { return Ok (()) ; } prctl_3args (PR_PAC_SET_ENABLED_KEYS , affected_keys as usize as * mut _ , enabled_keys as usize as * mut _ ,) . map (| _r | ()) }
    };
}

configure_pointer_authentication_keys!();