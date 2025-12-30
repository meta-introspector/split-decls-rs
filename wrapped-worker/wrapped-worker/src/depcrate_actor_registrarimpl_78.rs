// Generated macro for impl_78 (impl)
macro_rules! Depcrate_actor_registrarimpl_78 {
() => {
// Module: crate::actor::registrar
// Provides: {"impl_78"}
// Dependencies: {}
impl < W , CODEC > WorkerRegistrar < W , CODEC > where W : Worker + 'static , CODEC : Codec , { pub (crate) fn new () -> Self { Self { _marker : PhantomData , } } # [doc = " Sets a new message encoding."] pub fn encoding < C > (& self) -> WorkerRegistrar < W , C > where C : Codec , { WorkerRegistrar :: new () } # [doc = " Executes an worker in the current environment."] pub fn register (& self) where CODEC : Codec , W :: Input : Serialize + for < 'de > Deserialize < 'de > , W :: Output : Serialize + for < 'de > Deserialize < 'de > , { let scope = WorkerScope :: < W > :: new :: < CODEC > () ; let upd = WorkerLifecycleEvent :: Create (scope . clone ()) ; scope . send (upd) ; let handler = move | msg : ToWorker < W > | { let upd = WorkerLifecycleEvent :: Remote (msg) ; scope . send (upd) ; } ; let loaded : FromWorker < W > = FromWorker :: WorkerLoaded ; let worker = DedicatedWorker :: worker_self () ; worker . set_on_packed_message :: < _ , CODEC , _ > (handler) ; worker . post_packed_message :: < _ , CODEC > (loaded) ; } }
};
}
