// Generated macro for EtwGetTraitFromProviderTraits (function)
macro_rules! Depcrate_um_evntconsEtwGetTraitFromProviderTraits {
() => {
// Module: crate::um::evntcons
// Provides: {"EtwGetTraitFromProviderTraits"}
// Dependencies: {}
# [inline] pub unsafe fn EtwGetTraitFromProviderTraits (ProviderTraits : PVOID , TraitType : UCHAR , Trait : * mut PVOID , Size : PUSHORT ,) { use core :: ptr :: null_mut ; let ByteCount = read_unaligned (ProviderTraits as * mut USHORT) as isize ; let mut Ptr = ProviderTraits as PUCHAR ; let PtrEnd = Ptr . offset (ByteCount) ; * Trait = null_mut () ; * Size = 0 ; if ByteCount < 3 { return ; } Ptr = Ptr . offset (2) ; Ptr = Ptr . offset (strnlen (Ptr as PCSTR , (ByteCount - 3) as isize)) ; Ptr = Ptr . offset (1) ; while Ptr < PtrEnd { let TraitByteCount = read_unaligned (Ptr as * const USHORT) ; if TraitByteCount < 3 { return ; } if * Ptr . offset (2) == TraitType && Ptr . offset (TraitByteCount as isize) <= PtrEnd { * Trait = Ptr . offset (3) as PVOID ; * Size = TraitByteCount - 3 ; return ; } Ptr = Ptr . offset (TraitByteCount as isize) ; } }
};
}
