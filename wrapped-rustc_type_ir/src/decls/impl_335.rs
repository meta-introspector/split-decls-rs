macro_rules! deps {
    () => {
        Interner!();
        OpaqueTypeKey!();
        GenericArg!();
        Region!();
        GenericArgKind!();
    };
}

macro_rules! impl_335 {
    () => {
        deps!();
        impl < I : Interner > OpaqueTypeKey < I > { pub fn iter_captured_args (self , cx : I) -> impl Iterator < Item = (usize , I :: GenericArg) > { let variances = cx . variances_of (self . def_id . into ()) ; std :: iter :: zip (self . args . iter () , variances . iter ()) . enumerate () . filter_map (| (i , (arg , v)) | match (arg . kind () , v) { (_ , ty :: Invariant) => Some ((i , arg)) , (ty :: GenericArgKind :: Lifetime (_) , ty :: Bivariant) => None , _ => panic ! ("unexpected opaque type arg variance") , } ,) } pub fn fold_captured_lifetime_args (self , cx : I , mut f : impl FnMut (I :: Region) -> I :: Region ,) -> Self { let Self { def_id , args } = self ; let variances = cx . variances_of (def_id . into ()) ; let args = std :: iter :: zip (args . iter () , variances . iter ()) . map (| (arg , v) | match (arg . kind () , v) { (ty :: GenericArgKind :: Lifetime (_) , ty :: Bivariant) => arg , (ty :: GenericArgKind :: Lifetime (lt) , _) => f (lt) . into () , _ => arg , }) ; let args = cx . mk_args_from_iter (args) ; Self { def_id , args } } }
    };
}

impl_335!();