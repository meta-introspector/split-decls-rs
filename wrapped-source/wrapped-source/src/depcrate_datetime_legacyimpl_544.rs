// Generated macro for impl_544 (impl)
macro_rules! Depcrate_datetime_legacyimpl_544 {
() => {
// Module: crate::datetime::legacy
// Provides: {"impl_544"}
// Dependencies: {}
impl < 'a > From < & day_periods :: Symbols < 'a > > for LinearNames < 'a > { fn from (other : & day_periods :: Symbols < 'a >) -> Self { let vec : alloc :: vec :: Vec < & str > = match (other . noon . as_ref () , other . midnight . as_ref ()) { (Some (noon) , Some (midnight)) => vec ! [& other . am , & other . pm , & noon , & midnight] , (Some (noon) , None) => vec ! [& other . am , & other . pm , & noon] , (None , Some (midnight)) => vec ! [& other . am , & other . pm , "" , & midnight] , (None , None) => vec ! [& other . am , & other . pm] , } ; LinearNames { names : (& vec) . into () , } } }
};
}
