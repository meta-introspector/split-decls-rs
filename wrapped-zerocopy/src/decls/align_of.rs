macro_rules! align_of {
    () => {
        # [doc = " Computes alignment of `$ty: ?Sized`."] # [doc = ""] # [doc = " `align_of!` produces code which is valid in a `const` context."] # [cfg (__ZEROCOPY_INTERNAL_USE_ONLY_NIGHTLY_FEATURES_IN_TESTS)] # [doc (hidden)] # [macro_export] macro_rules ! align_of { ($ ty : ty) => { { # [repr (C)] struct OffsetOfTrailingIsAlignment { _byte : u8 , _trailing : $ ty , } trailing_field_offset ! (OffsetOfTrailingIsAlignment , _trailing) } } ; }
    };
}

align_of!();