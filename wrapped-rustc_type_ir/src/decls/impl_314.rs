macro_rules! deps {
    () => {
        TypeFolder!();
        Interner!();
        Binder!();
        Ty!();
        Predicate!();
        Region!();
        RegionFolder!();
        Const!();
        TypeFoldable!();
    };
}

macro_rules! impl_314 {
    () => {
        deps!();
        impl < I , F > TypeFolder < I > for RegionFolder < I , F > where I : Interner , F : FnMut (I :: Region , ty :: DebruijnIndex) -> I :: Region , { fn cx (& self) -> I { self . cx } fn fold_binder < T : TypeFoldable < I > > (& mut self , t : ty :: Binder < I , T >) -> ty :: Binder < I , T > { self . current_index . shift_in (1) ; let t = t . super_fold_with (self) ; self . current_index . shift_out (1) ; t } # [instrument (skip (self) , level = "debug" , ret)] fn fold_region (& mut self , r : I :: Region) -> I :: Region { match r . kind () { ty :: ReBound (debruijn , _) if debruijn < self . current_index => { debug ! (? self . current_index , "skipped bound region") ; r } _ => { debug ! (? self . current_index , "folding free region") ; (self . fold_region_fn) (r , self . current_index) } } } fn fold_ty (& mut self , t : I :: Ty) -> I :: Ty { if t . has_type_flags (TypeFlags :: HAS_FREE_REGIONS | TypeFlags :: HAS_RE_BOUND | TypeFlags :: HAS_RE_ERASED ,) { t . super_fold_with (self) } else { t } } fn fold_const (& mut self , ct : I :: Const) -> I :: Const { if ct . has_type_flags (TypeFlags :: HAS_FREE_REGIONS | TypeFlags :: HAS_RE_BOUND | TypeFlags :: HAS_RE_ERASED ,) { ct . super_fold_with (self) } else { ct } } fn fold_predicate (& mut self , p : I :: Predicate) -> I :: Predicate { if p . has_type_flags (TypeFlags :: HAS_FREE_REGIONS | TypeFlags :: HAS_RE_BOUND | TypeFlags :: HAS_RE_ERASED ,) { p . super_fold_with (self) } else { p } } }
    };
}

impl_314!();