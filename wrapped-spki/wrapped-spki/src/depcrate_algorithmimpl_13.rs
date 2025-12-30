// Generated macro for impl_13 (impl)
macro_rules! Depcrate_algorithmimpl_13 {
() => {
// Module: crate::algorithm
// Provides: {"impl_13"}
// Dependencies: {}
impl < Params > ValueOrd for AlgorithmIdentifier < Params > where Params : DerOrd , { fn value_cmp (& self , other : & Self) -> der :: Result < Ordering > { match self . oid . der_cmp (& other . oid) ? { Ordering :: Equal => self . parameters . der_cmp (& other . parameters) , other => Ok (other) , } } }
};
}
