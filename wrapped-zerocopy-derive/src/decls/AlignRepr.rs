macro_rules! AlignRepr {
    () => {
        # [doc = " `repr(packed(...))` or `repr(align(...))`"] # [cfg_attr (test , derive (Copy , Clone , Debug , Eq , PartialEq))] pub (crate) enum AlignRepr < Packed > { Packed (Packed) , Align (NonZeroU32) , }
    };
}

AlignRepr!();