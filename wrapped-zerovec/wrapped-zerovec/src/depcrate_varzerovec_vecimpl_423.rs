// Generated macro for impl_423 (impl)
macro_rules! Depcrate_varzerovec_vecimpl_423 {
() => {
// Module: crate::varzerovec::vec
// Provides: {"impl_423"}
// Dependencies: {}
impl < 'a , 'b , T , F > PartialEq < VarZeroVec < 'b , T , F > > for VarZeroVec < 'a , T , F > where T : VarULE , T : ? Sized , T : PartialEq , F : VarZeroVecFormat , { # [inline] fn eq (& self , other : & VarZeroVec < 'b , T , F >) -> bool { if self . is_empty () || other . is_empty () { return self . is_empty () && other . is_empty () ; } self . as_bytes () . eq (other . as_bytes ()) } }
};
}
