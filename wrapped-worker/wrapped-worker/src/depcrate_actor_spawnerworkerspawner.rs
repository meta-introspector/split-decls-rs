// Generated macro for WorkerSpawner (struct)
macro_rules! Depcrate_actor_spawnerWorkerSpawner {
() => {
// Module: crate::actor::spawner
// Provides: {"WorkerSpawner"}
// Dependencies: {}
# [doc = " A spawner to create workers."] # [derive (Clone)] pub struct WorkerSpawner < W , CODEC = Bincode > where W : Worker , CODEC : Codec , { _marker : PhantomData < (W , CODEC) > , callback : Option < Callback < W :: Output > > , }
};
}
