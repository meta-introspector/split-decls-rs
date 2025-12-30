// Generated macro for impl_182 (impl)
macro_rules! Depcrate_oneshot_spawnerimpl_182 {
() => {
// Module: crate::oneshot::spawner
// Provides: {"impl_182"}
// Dependencies: {}
impl < N , CODEC > OneshotSpawner < N , CODEC > where N : Oneshot + 'static , CODEC : Codec , { # [doc = " Creates a [OneshotSpawner]."] pub const fn new () -> Self { Self { inner : WorkerSpawner :: < OneshotWorker < N > , CODEC > :: new () , } } # [doc = " Sets a new message encoding."] pub const fn encoding < C > (& self) -> OneshotSpawner < N , C > where C : Codec , { OneshotSpawner { inner : WorkerSpawner :: < OneshotWorker < N > , C > :: new () , } } # [doc = " Spawns an Oneshot Worker."] pub fn spawn (mut self , path : & str) -> OneshotBridge < N > where N :: Input : Serialize + for < 'de > Deserialize < 'de > , N :: Output : Serialize + for < 'de > Deserialize < 'de > , { let rx = OneshotBridge :: register_callback (& mut self . inner) ; let inner = self . inner . spawn (path) ; OneshotBridge :: new (inner , rx) } # [doc = " Spawns an Oneshot Worker with a loader shim script."] pub fn spawn_with_loader (mut self , loader_path : & str) -> OneshotBridge < N > where N :: Input : Serialize + for < 'de > Deserialize < 'de > , N :: Output : Serialize + for < 'de > Deserialize < 'de > , { let rx = OneshotBridge :: register_callback (& mut self . inner) ; let inner = self . inner . spawn_with_loader (loader_path) ; OneshotBridge :: new (inner , rx) } }
};
}
