// Generated macro for impl_618 (impl)
macro_rules! Depcrate_steerimpl_618 {
() => {
// Module: crate::steer
// Provides: {"impl_618"}
// Dependencies: {}
impl < S , F , Req > Clone for Steer < S , F , Req > where S : Clone , F : Clone , { fn clone (& self) -> Self { Self { router : self . router . clone () , services : self . services . clone () , not_ready : self . not_ready . clone () , _phantom : PhantomData , } } }
};
}
