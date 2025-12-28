macro_rules! deps {
    () => {
        AlignRepr!();
    };
}

macro_rules! FromRawReprsError {
    () => {
        deps!();
        # [doc = " The error from extracting a high-level repr type from a list of `RawRepr`s."] # [cfg_attr (test , derive (Copy , Clone , Debug , Eq , PartialEq))] enum FromRawReprsError < E > { # [doc = " One of the `RawRepr`s is invalid for the high-level repr we're parsing"] # [doc = " (e.g. there's a `packed` repr and we're parsing an `AlignRepr` for an"] # [doc = " enum type)."] Single (E) , # [doc = " Two `RawRepr`s appear which both affect the high-level repr we're"] # [doc = " parsing (e.g., the list is `#[repr(align(2), packed)]`). Note that we"] # [doc = " conservatively treat redundant reprs as conflicting (e.g."] # [doc = " `#[repr(packed, packed)]`)."] Conflict , }
    };
}

FromRawReprsError!();