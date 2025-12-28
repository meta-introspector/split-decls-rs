macro_rules! deps {
    () => {
        Ty!();
        PlaceholderConst!();
        GenericArgKind!();
        Region!();
        GenericArg!();
        ConstKind!();
        CanonicalVarValues!();
        Const!();
        CanonicalVarKind!();
        Interner!();
    };
}

macro_rules! impl_271 {
    () => {
        deps!();
        impl < I : Interner > CanonicalVarValues < I > { pub fn is_identity (& self) -> bool { self . var_values . iter () . enumerate () . all (| (bv , arg) | match arg . kind () { ty :: GenericArgKind :: Lifetime (r) => { matches ! (r . kind () , ty :: ReBound (ty :: INNERMOST , br) if br . var () . as_usize () == bv) } ty :: GenericArgKind :: Type (ty) => { matches ! (ty . kind () , ty :: Bound (ty :: INNERMOST , bt) if bt . var () . as_usize () == bv) } ty :: GenericArgKind :: Const (ct) => { matches ! (ct . kind () , ty :: ConstKind :: Bound (ty :: INNERMOST , bc) if bc . var () . as_usize () == bv) } }) } pub fn is_identity_modulo_regions (& self) -> bool { let mut var = ty :: BoundVar :: ZERO ; for arg in self . var_values . iter () { match arg . kind () { ty :: GenericArgKind :: Lifetime (r) => { if matches ! (r . kind () , ty :: ReBound (ty :: INNERMOST , br) if var == br . var ()) { var = var + 1 ; } else { } } ty :: GenericArgKind :: Type (ty) => { if matches ! (ty . kind () , ty :: Bound (ty :: INNERMOST , bt) if var == bt . var ()) { var = var + 1 ; } else { return false ; } } ty :: GenericArgKind :: Const (ct) => { if matches ! (ct . kind () , ty :: ConstKind :: Bound (ty :: INNERMOST , bc) if var == bc . var ()) { var = var + 1 ; } else { return false ; } } } } true } pub fn make_identity (cx : I , infos : I :: CanonicalVarKinds) -> CanonicalVarValues < I > { CanonicalVarValues { var_values : cx . mk_args_from_iter (infos . iter () . enumerate () . map (| (i , kind) | -> I :: GenericArg { match kind { CanonicalVarKind :: Ty { .. } | CanonicalVarKind :: Int | CanonicalVarKind :: Float | CanonicalVarKind :: PlaceholderTy (_) => { Ty :: new_anon_bound (cx , ty :: INNERMOST , ty :: BoundVar :: from_usize (i)) . into () } CanonicalVarKind :: Region (_) | CanonicalVarKind :: PlaceholderRegion (_) => { Region :: new_anon_bound (cx , ty :: INNERMOST , ty :: BoundVar :: from_usize (i)) . into () } CanonicalVarKind :: Const (_) | CanonicalVarKind :: PlaceholderConst (_) => { Const :: new_anon_bound (cx , ty :: INNERMOST , ty :: BoundVar :: from_usize (i)) . into () } } } ,)) , } } # [doc = " Creates dummy var values which should not be used in a"] # [doc = " canonical response."] pub fn dummy () -> CanonicalVarValues < I > { CanonicalVarValues { var_values : Default :: default () } } pub fn instantiate (cx : I , variables : I :: CanonicalVarKinds , mut f : impl FnMut (& [I :: GenericArg] , CanonicalVarKind < I >) -> I :: GenericArg ,) -> CanonicalVarValues < I > { if variables . len () <= 4 { let mut var_values = ArrayVec :: < _ , 4 > :: new () ; for info in variables . iter () { var_values . push (f (& var_values , info)) ; } CanonicalVarValues { var_values : cx . mk_args (& var_values) } } else { CanonicalVarValues :: instantiate_cold (cx , variables , f) } } # [cold] fn instantiate_cold (cx : I , variables : I :: CanonicalVarKinds , mut f : impl FnMut (& [I :: GenericArg] , CanonicalVarKind < I >) -> I :: GenericArg ,) -> CanonicalVarValues < I > { let mut var_values = Vec :: with_capacity (variables . len ()) ; for info in variables . iter () { var_values . push (f (& var_values , info)) ; } CanonicalVarValues { var_values : cx . mk_args (& var_values) } } # [inline] pub fn len (& self) -> usize { self . var_values . len () } }
    };
}

impl_271!()