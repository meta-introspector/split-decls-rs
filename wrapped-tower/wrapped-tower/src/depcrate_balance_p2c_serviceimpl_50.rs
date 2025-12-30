// Generated macro for impl_50 (impl)
macro_rules! Depcrate_balance_p2c_serviceimpl_50 {
() => {
// Module: crate::balance::p2c::service
// Provides: {"impl_50"}
// Dependencies: {}
impl < D , Req > Balance < D , Req > where D : Discover , D :: Key : Hash , D :: Service : Service < Req > , < D :: Service as Service < Req > > :: Error : Into < crate :: BoxError > , { # [doc = " Constructs a load balancer that uses operating system entropy."] pub fn new (discover : D) -> Self { Self :: from_rng (discover , HasherRng :: default ()) } # [doc = " Constructs a load balancer seeded with the provided random number generator."] pub fn from_rng < R : Rng + Send + Sync + 'static > (discover : D , rng : R) -> Self { let rng = Box :: new (rng) ; Self { rng , discover , services : ReadyCache :: default () , ready_index : None , _req : PhantomData , } } # [doc = " Returns the number of endpoints currently tracked by the balancer."] pub fn len (& self) -> usize { self . services . len () } # [doc = " Returns whether or not the balancer is empty."] pub fn is_empty (& self) -> bool { self . services . is_empty () } }
};
}
