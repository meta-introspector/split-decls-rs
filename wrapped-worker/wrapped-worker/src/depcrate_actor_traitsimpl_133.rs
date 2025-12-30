// Generated macro for impl_133 (impl)
macro_rules! Depcrate_actor_traitsimpl_133 {
() => {
// Module: crate::actor::traits
// Provides: {"impl_133"}
// Dependencies: {}
impl < W > Registrable for W where W : Worker + 'static , { type Registrar = WorkerRegistrar < Self > ; fn registrar () -> WorkerRegistrar < Self > { WorkerRegistrar :: new () } }
};
}
