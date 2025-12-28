macro_rules! deps {
    () => {
        CompoundRepr!();
        AlignRepr!();
        RawRepr!();
    };
}

macro_rules! FromRawReprError {
    () => {
        deps!();
        # [doc = " The error from converting from a `RawRepr`."] # [cfg_attr (test , derive (Debug , Eq , PartialEq))] pub (crate) enum FromRawReprError < E > { # [doc = " The `RawRepr` doesn't affect the high-level repr we're parsing (e.g."] # [doc = " it's `align(...)` and we're parsing a `CompoundRepr`)."] None , # [doc = " The `RawRepr` is invalid for the high-level repr we're parsing (e.g."] # [doc = " it's `packed` repr and we're parsing an `AlignRepr` for an enum type)."] Err (E) , }
    };
}

FromRawReprError!();