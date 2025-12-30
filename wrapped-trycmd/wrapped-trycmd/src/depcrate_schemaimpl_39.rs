// Generated macro for impl_39 (impl)
macro_rules! Depcrate_schemaimpl_39 {
() => {
// Module: crate::schema
// Provides: {"impl_39"}
// Dependencies: {}
impl < P , E > From < Result < P , E > > for Bin where P : Into < Bin > , E : std :: fmt :: Display , { fn from (other : Result < P , E >) -> Self { match other { Ok (path) => path . into () , Err (err) => { let err = crate :: Error :: new (err . to_string ()) ; Bin :: Error (err) } } } }
};
}
