// Generated macro for impl_346 (impl)
macro_rules! Depcrate_exprimpl_346 {
() => {
// Module: crate::expr
// Provides: {"impl_346"}
// Dependencies: {}
# [cfg (feature = "printing")] impl IdentFragment for Member { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { match self { Member :: Named (m) => Display :: fmt (m , formatter) , Member :: Unnamed (m) => Display :: fmt (& m . index , formatter) , } } fn span (& self) -> Option < Span > { match self { Member :: Named (m) => Some (m . span ()) , Member :: Unnamed (m) => Some (m . span) , } } }
};
}
