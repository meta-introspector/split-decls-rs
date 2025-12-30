// Generated macro for impl_348 (impl)
macro_rules! Depcrate_exprimpl_348 {
() => {
// Module: crate::expr
// Provides: {"impl_348"}
// Dependencies: {}
# [cfg (feature = "printing")] impl IdentFragment for Member { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { match self { Member :: Named (m) => Display :: fmt (m , formatter) , Member :: Unnamed (m) => Display :: fmt (& m . index , formatter) , } } fn span (& self) -> Option < Span > { match self { Member :: Named (m) => Some (m . span ()) , Member :: Unnamed (m) => Some (m . span) , } } }
};
}
