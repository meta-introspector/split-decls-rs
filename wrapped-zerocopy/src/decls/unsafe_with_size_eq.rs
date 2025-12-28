macro_rules! deps {
    () => {
        FromBytes!();
        IntoBytes!();
        InvariantsEq!();
        PointerMetadata!();
        TryFromBytes!();
        Immutable!();
        SizeEq!();
        KnownLayout!();
        FromZeros!();
    };
}

macro_rules! unsafe_with_size_eq {
    () => {
        deps!();
        # [doc = " Invokes `$blk` in a context in which `$src<$t>` and `$dst<$u>` implement"] # [doc = " `SizeEq`."] # [doc = ""] # [doc = " This macro emits code which implements `SizeEq`, and ensures that the impl"] # [doc = " is sound via PME."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Inside of `$blk`, the caller must only use `$src` and `$dst` as `$src<$t>`"] # [doc = " and `$dst<$u>`. The caller must not use `$src` or `$dst` to wrap any other"] # [doc = " types."] macro_rules ! unsafe_with_size_eq { (<$ src : ident <$ t : ident >, $ dst : ident <$ u : ident >> $ blk : expr) => { { crate :: util :: macros :: __unsafe () ; use crate :: { KnownLayout , pointer :: PtrInner } ; # [repr (transparent)] struct $ src < T : ? Sized > (T) ; # [repr (transparent)] struct $ dst < U : ? Sized > (U) ; unsafe_impl_for_transparent_wrapper ! (T : ? Sized => $ src < T >) ; unsafe_impl_for_transparent_wrapper ! (T : ? Sized => $ dst < T >) ; unsafe impl < T : ? Sized > InvariantsEq <$ src < T >> for T { } unsafe impl < T : ? Sized > InvariantsEq <$ dst < T >> for T { } unsafe impl < T : ? Sized , U : ? Sized > SizeEq <$ src < T >> for $ dst < U > where T : KnownLayout < PointerMetadata = usize >, U : KnownLayout < PointerMetadata = usize >, { fn cast_from_raw (src : PtrInner <'_ , $ src < T >>) -> PtrInner <'_ , Self > { let src : PtrInner <'_ , T > = unsafe { cast ! (src) } ; let dst : PtrInner <'_ , U > = crate :: layout :: cast_from_raw (src) ; unsafe { cast ! (dst) } } } if 1 == 0 { let ptr = <$ t as KnownLayout >:: raw_dangling () ; # [allow (unused_unsafe)] let ptr = unsafe { crate :: pointer :: PtrInner :: new (ptr) } ; # [allow (unused_unsafe)] let ptr = unsafe { cast ! (ptr) } ; let _ = <$ dst <$ u > as SizeEq <$ src <$ t >>>:: cast_from_raw (ptr) ; } impl_for_transmute_from ! (T : ? Sized + TryFromBytes => TryFromBytes for $ src < T > [< T >]) ; impl_for_transmute_from ! (T : ? Sized + FromBytes => FromBytes for $ src < T > [< T >]) ; impl_for_transmute_from ! (T : ? Sized + FromZeros => FromZeros for $ src < T > [< T >]) ; impl_for_transmute_from ! (T : ? Sized + IntoBytes => IntoBytes for $ src < T > [< T >]) ; impl_for_transmute_from ! (U : ? Sized + TryFromBytes => TryFromBytes for $ dst < U > [< U >]) ; impl_for_transmute_from ! (U : ? Sized + FromBytes => FromBytes for $ dst < U > [< U >]) ; impl_for_transmute_from ! (U : ? Sized + FromZeros => FromZeros for $ dst < U > [< U >]) ; impl_for_transmute_from ! (U : ? Sized + IntoBytes => IntoBytes for $ dst < U > [< U >]) ; unsafe_impl ! (T : ? Sized + Immutable => Immutable for $ src < T >) ; unsafe_impl ! (T : ? Sized + Immutable => Immutable for $ dst < T >) ; $ blk } } ; }
    };
}

unsafe_with_size_eq!();