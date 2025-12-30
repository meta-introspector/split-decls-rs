// Generated macro for OneshotSpawner (struct)
macro_rules! Depcrate_oneshot_spawnerOneshotSpawner {
() => {
// Module: crate::oneshot::spawner
// Provides: {"OneshotSpawner"}
// Dependencies: {}
# [doc = " A spawner to create oneshot workers."] # [derive (Debug , Default)] pub struct OneshotSpawner < N , CODEC = Bincode > where N : Oneshot + 'static , CODEC : Codec , { inner : WorkerSpawner < OneshotWorker < N > , CODEC > , }
};
}
