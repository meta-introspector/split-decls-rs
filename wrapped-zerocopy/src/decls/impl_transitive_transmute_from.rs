macro_rules! deps {
    () => {
        Valid!();
        TransmuteFrom!();
        SizeEq!();
    };
}

macro_rules! impl_transitive_transmute_from {
    () => {
        deps!();
        macro_rules ! impl_transitive_transmute_from { ($ ($ tyvar : ident $ (: ?$ optbound : ident) ?) ? => $ t : ty => $ u : ty => $ v : ty) => { const _ : () = { use crate :: pointer :: { TransmuteFrom , PtrInner , SizeEq , invariant :: Valid } ; unsafe impl <$ ($ tyvar $ (: ?$ optbound) ?) ?> SizeEq <$ t > for $ v where $ u : SizeEq <$ t >, $ v : SizeEq <$ u >, { # [inline (always)] fn cast_from_raw (t : PtrInner <'_ , $ t >) -> PtrInner <'_ , $ v > { let u = <$ u as SizeEq < _ >>:: cast_from_raw (t) ; <$ v as SizeEq < _ >>:: cast_from_raw (u) } } unsafe impl <$ ($ tyvar $ (: ?$ optbound) ?) ?> TransmuteFrom <$ t , Valid , Valid > for $ v where $ u : TransmuteFrom <$ t , Valid , Valid >, $ v : TransmuteFrom <$ u , Valid , Valid >, { } } ; } ; }
    };
}

impl_transitive_transmute_from!();