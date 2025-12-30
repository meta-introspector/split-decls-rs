// Generated macro for impl_619 (impl)
macro_rules! Depcrate_steerimpl_619 {
() => {
// Module: crate::steer
// Provides: {"impl_619"}
// Dependencies: {}
impl < S , F , Req > fmt :: Debug for Steer < S , F , Req > where S : fmt :: Debug , F : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let Self { router , services , not_ready , _phantom , } = self ; f . debug_struct ("Steer") . field ("router" , router) . field ("services" , services) . field ("not_ready" , not_ready) . finish () } }
};
}
