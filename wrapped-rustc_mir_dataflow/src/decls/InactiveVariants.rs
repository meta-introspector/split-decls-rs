macro_rules! InactiveVariants {
    () => {
        # [doc = " Indicates which variants are inactive at a `SwitchInt` edge by listing their `VariantIdx`s or"] # [doc = " specifying the single active variant's `VariantIdx`."] pub (crate) enum InactiveVariants { Inactives (SmallVec < [VariantIdx ; 4] >) , Active (VariantIdx) , }
    };
}

InactiveVariants!()