macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! if_typesize_details {
    () => {
        deps!();
        # [doc = " Passes through the given tokens if the `details` feature of `typesize` is enabled."] # [doc = ""] # [doc = " This is mainly useful for libaries making their own [`TypeSize`] to be compatible with `details` on or off."] # [macro_export] # [cfg (not (feature = "details"))] macro_rules ! if_typesize_details { ($ ($ tt : tt) *) => { } ; }
    };
}

if_typesize_details!();