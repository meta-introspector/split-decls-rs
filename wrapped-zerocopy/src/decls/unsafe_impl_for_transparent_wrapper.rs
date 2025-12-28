macro_rules! deps {
    () => {
        SizeEq!();
        Valid!();
        TransmuteFrom!();
    };
}

macro_rules! unsafe_impl_for_transparent_wrapper {
    () => {
        deps!();
        # [doc = " Implements `TransmuteFrom` and `SizeEq` for `T` and `$wrapper<T>`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `T` and `$wrapper<T>` must have the same bit validity, and must have the"] # [doc = " same size in the sense of `SizeEq`."] macro_rules ! unsafe_impl_for_transparent_wrapper { (T $ (: ?$ optbound : ident) ? => $ wrapper : ident < T >) => { { crate :: util :: macros :: __unsafe () ; use crate :: pointer :: { TransmuteFrom , PtrInner , SizeEq , invariant :: Valid } ; unsafe impl < T $ (: ?$ optbound) ?> TransmuteFrom < T , Valid , Valid > for $ wrapper < T > { } unsafe impl < T $ (: ?$ optbound) ?> TransmuteFrom <$ wrapper < T >, Valid , Valid > for T { } unsafe impl < T $ (: ?$ optbound) ?> SizeEq < T > for $ wrapper < T > { # [inline (always)] fn cast_from_raw (t : PtrInner <'_ , T >) -> PtrInner <'_ , $ wrapper < T >> { unsafe { cast ! (t) } } } unsafe impl < T $ (: ?$ optbound) ?> SizeEq <$ wrapper < T >> for T { # [inline (always)] fn cast_from_raw (t : PtrInner <'_ , $ wrapper < T >>) -> PtrInner <'_ , T > { unsafe { cast ! (t) } } } } } ; }
    };
}

unsafe_impl_for_transparent_wrapper!()