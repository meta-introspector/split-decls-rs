macro_rules! deps {
    () => {
        SizeInfo!();
        TrailingSliceLayout!();
        KnownLayout!();
        SliceDst!();
    };
}

macro_rules! static_assert_dst_is_not_zst {
    () => {
        deps!();
        # [doc = " Assert at compile time that `tyvar` does not have a zero-sized DST"] # [doc = " component."] macro_rules ! static_assert_dst_is_not_zst { ($ tyvar : ident) => { { use crate :: KnownLayout ; static_assert ! ($ tyvar : ? Sized + KnownLayout => { let dst_is_zst = match $ tyvar :: LAYOUT . size_info { crate :: SizeInfo :: Sized { .. } => false , crate :: SizeInfo :: SliceDst (TrailingSliceLayout { elem_size , .. }) => { elem_size == 0 } } ; ! dst_is_zst } , "cannot call this method on a dynamically-sized type whose trailing slice element is zero-sized") ; } } }
    };
}

static_assert_dst_is_not_zst!();