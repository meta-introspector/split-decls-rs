// Generated macro for impl_262 (impl)
macro_rules! Depcrate_reactor_spawnerimpl_262 {
() => {
// Module: crate::reactor::spawner
// Provides: {"impl_262"}
// Dependencies: {}
impl < R , CODEC > ReactorSpawner < R , CODEC > where R : Reactor + 'static , CODEC : Codec , { # [doc = " Creates a ReactorSpawner."] pub const fn new () -> Self { Self { inner : WorkerSpawner :: < ReactorWorker < R > , CODEC > :: new () , } } # [doc = " Sets a new message encoding."] pub const fn encoding < C > (& self) -> ReactorSpawner < R , C > where C : Codec , { ReactorSpawner { inner : WorkerSpawner :: < ReactorWorker < R > , C > :: new () , } } # [doc = " Spawns a reactor worker."] pub fn spawn (mut self , path : & str) -> ReactorBridge < R > where < R :: Scope as ReactorScoped > :: Input : Serialize + for < 'de > Deserialize < 'de > , < R :: Scope as ReactorScoped > :: Output : Serialize + for < 'de > Deserialize < 'de > , { let rx = ReactorBridge :: register_callback (& mut self . inner) ; let inner = self . inner . spawn (path) ; ReactorBridge :: new (inner , rx) } # [doc = " Spawns a Reactor Worker with a loader shim script."] pub fn spawn_with_loader (mut self , loader_path : & str) -> ReactorBridge < R > where < R :: Scope as ReactorScoped > :: Input : Serialize + for < 'de > Deserialize < 'de > , < R :: Scope as ReactorScoped > :: Output : Serialize + for < 'de > Deserialize < 'de > , { let rx = ReactorBridge :: register_callback (& mut self . inner) ; let inner = self . inner . spawn_with_loader (loader_path) ; ReactorBridge :: new (inner , rx) } }
};
}
