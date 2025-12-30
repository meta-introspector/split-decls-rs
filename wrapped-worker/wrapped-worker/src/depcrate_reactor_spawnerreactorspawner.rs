// Generated macro for ReactorSpawner (struct)
macro_rules! Depcrate_reactor_spawnerReactorSpawner {
() => {
// Module: crate::reactor::spawner
// Provides: {"ReactorSpawner"}
// Dependencies: {}
# [doc = " A spawner to create oneshot workers."] # [derive (Debug , Default)] pub struct ReactorSpawner < R , CODEC = Bincode > where R : Reactor + 'static , CODEC : Codec , { inner : WorkerSpawner < ReactorWorker < R > , CODEC > , }
};
}
