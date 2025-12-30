// Generated macro for impl_92 (impl)
macro_rules! Depcrate_patimpl_92 {
() => {
// Module: crate::pat
// Provides: {"impl_92"}
// Dependencies: {}
impl < Cx : PatCx > WitnessPat < Cx > { pub (crate) fn new (ctor : Constructor < Cx > , fields : Vec < Self > , ty : Cx :: Ty) -> Self { Self { ctor , fields , ty } } # [doc = " Create a wildcard pattern for this type. If the type is empty, we create a `!` pattern."] pub (crate) fn wildcard (cx : & Cx , ty : Cx :: Ty) -> Self { let is_empty = cx . ctors_for_ty (& ty) . is_ok_and (| ctors | ctors . all_empty ()) ; let ctor = if is_empty { Never } else { Wildcard } ; Self :: new (ctor , Vec :: new () , ty) } # [doc = " Construct a pattern that matches everything that starts with this constructor."] # [doc = " For example, if `ctor` is a `Constructor::Variant` for `Option::Some`, we get the pattern"] # [doc = " `Some(_)`."] pub (crate) fn wild_from_ctor (cx : & Cx , ctor : Constructor < Cx > , ty : Cx :: Ty) -> Self { if matches ! (ctor , Wildcard) { return Self :: wildcard (cx , ty) ; } let fields = cx . ctor_sub_tys (& ctor , & ty) . filter (| (_ , PrivateUninhabitedField (skip)) | ! skip) . map (| (ty , _) | Self :: wildcard (cx , ty)) . collect () ; Self :: new (ctor , fields , ty) } pub fn ctor (& self) -> & Constructor < Cx > { & self . ctor } pub fn ty (& self) -> & Cx :: Ty { & self . ty } pub fn is_never_pattern (& self) -> bool { match self . ctor () { Never => true , Or => self . fields . iter () . all (| p | p . is_never_pattern ()) , _ => self . fields . iter () . any (| p | p . is_never_pattern ()) , } } pub fn iter_fields (& self) -> impl Iterator < Item = & WitnessPat < Cx > > { self . fields . iter () } }
};
}
