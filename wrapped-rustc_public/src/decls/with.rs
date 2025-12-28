macro_rules! deps {
    () => {
        CompilerInterface!();
    };
}

macro_rules! with {
    () => {
        deps!();
        # [doc = " Execute the given function with access the [`CompilerInterface`]."] # [doc = ""] # [doc = " I.e., This function will load the current interface and calls a function with it."] # [doc = " Do not nest these, as that will ICE."] pub (crate) fn with < R > (f : impl FnOnce (& dyn CompilerInterface) -> R) -> R { assert ! (TLV . is_set ()) ; TLV . with (| tlv | { let ptr = tlv . get () ; assert ! (! ptr . is_null ()) ; f (unsafe { * (ptr as * const & dyn CompilerInterface) }) }) }
    };
}

with!()