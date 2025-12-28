macro_rules! deps {
    () => {
        TestReachabilityVisitor!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < 'a , 'tcx > TestReachabilityVisitor < 'a , 'tcx > { fn check_def_id (& self , owner_id : OwnerId) { self . effective_visibility_diagnostic (owner_id . def_id) ; match self . tcx . def_kind (owner_id) { DefKind :: Enum => { let def = self . tcx . adt_def (owner_id . def_id) ; for variant in def . variants () { self . effective_visibility_diagnostic (variant . def_id . expect_local ()) ; if let Some (ctor_def_id) = variant . ctor_def_id () { self . effective_visibility_diagnostic (ctor_def_id . expect_local ()) ; } for field in & variant . fields { self . effective_visibility_diagnostic (field . did . expect_local ()) ; } } } DefKind :: Struct | DefKind :: Union => { let def = self . tcx . adt_def (owner_id . def_id) . non_enum_variant () ; if let Some (ctor_def_id) = def . ctor_def_id () { self . effective_visibility_diagnostic (ctor_def_id . expect_local ()) ; } for field in & def . fields { self . effective_visibility_diagnostic (field . did . expect_local ()) ; } } _ => { } } } }
    };
}

impl_22!()