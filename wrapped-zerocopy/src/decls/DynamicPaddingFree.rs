macro_rules! deps {
    () => {
        IntoBytes!();
    };
}

macro_rules! DynamicPaddingFree {
    () => {
        deps!();
        # [cfg_attr (zerocopy_diagnostic_on_unimplemented_1_78_0 , diagnostic :: on_unimplemented (message = "`{T}` has one or more padding bytes" , label = "types with padding cannot implement `IntoBytes`" , note = "consider using `zerocopy::Unalign` to lower the alignment of individual fields" , note = "consider adding explicit fields where padding would be" , note = "consider using `#[repr(packed)]` to remove padding"))] pub trait DynamicPaddingFree < T : ? Sized , const HAS_PADDING : bool > { }
    };
}

DynamicPaddingFree!();