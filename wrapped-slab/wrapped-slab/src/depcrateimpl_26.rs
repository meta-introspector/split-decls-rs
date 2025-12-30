// Generated macro for impl_26 (impl)
macro_rules! Depcrateimpl_26 {
() => {
// Module: crate
// Provides: {"impl_26"}
// Dependencies: {}
impl fmt :: Display for GetDisjointMutError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let msg = match self { GetDisjointMutError :: IndexVacant => "an index is vacant" , GetDisjointMutError :: IndexOutOfBounds => "an index is out of bounds" , GetDisjointMutError :: OverlappingIndices => "there were overlapping indices" , } ; fmt :: Display :: fmt (msg , f) } }
};
}
