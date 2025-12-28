macro_rules! deps {
    () => {
        Constructor!();
        ConstructorSet!();
        PlaceInfo!();
        PatCx!();
        PrivateUninhabitedField!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < Cx : PatCx > PlaceInfo < Cx > { # [doc = " Given a constructor for the current place, we return one `PlaceInfo` for each field of the"] # [doc = " constructor."] fn specialize (& self , cx : & Cx , ctor : & Constructor < Cx > ,) -> impl Iterator < Item = Self > + ExactSizeIterator { let ctor_sub_tys = cx . ctor_sub_tys (ctor , & self . ty) ; let ctor_sub_validity = self . validity . specialize (ctor) ; ctor_sub_tys . map (move | (ty , PrivateUninhabitedField (private_uninhabited)) | PlaceInfo { ty , private_uninhabited , validity : ctor_sub_validity , is_scrutinee : false , }) } # [doc = " This analyzes a column of constructors corresponding to the current place. It returns a pair"] # [doc = " `(split_ctors, missing_ctors)`."] # [doc = ""] # [doc = " `split_ctors` is a splitted list of constructors that cover the whole type. This will be"] # [doc = " used to specialize the matrix."] # [doc = ""] # [doc = " `missing_ctors` is a list of the constructors not found in the column, for reporting"] # [doc = " purposes."] fn split_column_ctors < 'a > (& self , cx : & Cx , ctors : impl Iterator < Item = & 'a Constructor < Cx > > + Clone ,) -> Result < (SmallVec < [Constructor < Cx > ; 1] > , Vec < Constructor < Cx > >) , Cx :: Error > where Cx : 'a , { debug ! (? self . ty) ; if self . private_uninhabited { return Ok ((smallvec ! [Constructor :: PrivateUninhabited] , vec ! [])) ; } if ctors . clone () . any (| c | matches ! (c , Constructor :: Or)) { return Ok ((smallvec ! [Constructor :: Or] , vec ! [])) ; } let ctors_for_ty = cx . ctors_for_ty (& self . ty) ? ; debug ! (? ctors_for_ty) ; let is_toplevel_exception = self . is_scrutinee && matches ! (ctors_for_ty , ConstructorSet :: NoConstructors) ; let empty_arms_are_unreachable = self . validity . is_known_valid () && (is_toplevel_exception || cx . is_exhaustive_patterns_feature_on ()) ; let can_omit_empty_arms = self . validity . is_known_valid () || is_toplevel_exception || cx . is_exhaustive_patterns_feature_on () ; let mut split_set = ctors_for_ty . split (ctors) ; debug ! (? split_set) ; let all_missing = split_set . present . is_empty () ; let mut split_ctors = split_set . present ; if ! (split_set . missing . is_empty () && (split_set . missing_empty . is_empty () || empty_arms_are_unreachable)) { split_ctors . push (Constructor :: Missing) ; } let mut missing_ctors = split_set . missing ; if ! can_omit_empty_arms { missing_ctors . append (& mut split_set . missing_empty) ; } let report_individual_missing_ctors = self . is_scrutinee || ! all_missing ; if ! missing_ctors . is_empty () && ! report_individual_missing_ctors { missing_ctors = vec ! [Constructor :: Wildcard] ; } else if missing_ctors . iter () . any (| c | c . is_non_exhaustive ()) && ! cx . exhaustive_witnesses () { missing_ctors = vec ! [Constructor :: NonExhaustive] ; } Ok ((split_ctors , missing_ctors)) } }
    };
}

impl_101!()