// Generated macro for impl_629 (impl)
macro_rules! Depcrate_ule_nicheimpl_629 {
() => {
// Module: crate::ule::niche
// Provides: {"impl_629"}
// Dependencies: {}
impl < U : NicheBytes < N > + ULE , const N : usize > NichedOptionULE < U , N > { # [doc = " New `NichedOptionULE<U, N>` from `Option<U>`"] pub fn new (opt : Option < U >) -> Self { assert ! (N == core :: mem :: size_of ::< U > ()) ; match opt { Some (u) => Self { valid : u } , None => Self { niche : < U as NicheBytes < N > > :: NICHE_BIT_PATTERN , } , } } # [doc = " Convert to an `Option<U>`"] pub fn get (self) -> Option < U > { unsafe { if self . niche == < U as NicheBytes < N > > :: NICHE_BIT_PATTERN { None } else { Some (self . valid) } } } # [doc = " Borrows as an `Option<&U>`."] pub fn as_ref (& self) -> Option < & U > { unsafe { if self . niche == < U as NicheBytes < N > > :: NICHE_BIT_PATTERN { None } else { Some (& self . valid) } } } }
};
}
