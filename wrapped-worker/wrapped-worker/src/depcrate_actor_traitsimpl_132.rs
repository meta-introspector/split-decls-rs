// Generated macro for impl_132 (impl)
macro_rules! Depcrate_actor_traitsimpl_132 {
() => {
// Module: crate::actor::traits
// Provides: {"impl_132"}
// Dependencies: {}
impl < W > Spawnable for W where W : Worker + 'static , { type Spawner = WorkerSpawner < Self > ; fn spawner () -> WorkerSpawner < Self > { WorkerSpawner :: new () } }
};
}
