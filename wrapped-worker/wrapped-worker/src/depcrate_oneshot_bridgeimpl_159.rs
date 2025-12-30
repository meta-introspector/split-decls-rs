// Generated macro for impl_159 (impl)
macro_rules! Depcrate_oneshot_bridgeimpl_159 {
() => {
// Module: crate::oneshot::bridge
// Provides: {"impl_159"}
// Dependencies: {}
impl < N > OneshotBridge < N > where N : Oneshot + 'static , { # [inline (always)] pub (crate) fn new (inner : WorkerBridge < OneshotWorker < N > > , rx : UnboundedReceiver < N :: Output > ,) -> Self { Self { inner , rx } } # [inline (always)] pub (crate) fn register_callback < CODEC > (spawner : & mut WorkerSpawner < OneshotWorker < N > , CODEC > ,) -> UnboundedReceiver < N :: Output > where CODEC : Codec , { let (tx , rx) = mpsc :: unbounded () ; spawner . callback (move | output | { let _ = tx . send_now (output) ; }) ; rx } # [doc = " Forks the bridge."] # [doc = ""] # [doc = " This method creates a new bridge that can be used to execute tasks on the same worker instance."] pub fn fork (& self) -> Self { let (tx , rx) = mpsc :: unbounded () ; let inner = self . inner . fork (Some (move | output | { let _ = tx . send_now (output) ; })) ; Self { inner , rx } } # [doc = " Run the the current oneshot worker once in the current worker instance."] pub async fn run (& mut self , input : N :: Input) -> N :: Output { self . inner . send (input) ; self . rx . next () . await . expect ("failed to receive result from worker") } }
};
}
