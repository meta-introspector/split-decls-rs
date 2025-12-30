// Generated macro for impl_215 (impl)
macro_rules! Depcrate_reactor_bridgeimpl_215 {
() => {
// Module: crate::reactor::bridge
// Provides: {"impl_215"}
// Dependencies: {}
impl < R > ReactorBridge < R > where R : Reactor + 'static , { # [inline (always)] pub (crate) fn new (inner : WorkerBridge < ReactorWorker < R > > , rx : UnboundedReceiver < < R :: Scope as ReactorScoped > :: Output > ,) -> Self { Self { inner , rx } } pub (crate) fn output_callback (tx : & UnboundedSender < < R :: Scope as ReactorScoped > :: Output > , output : ReactorOutput < < R :: Scope as ReactorScoped > :: Output > ,) { match output { ReactorOutput :: Output (m) => { let _ = tx . send_now (m) ; } ReactorOutput :: Finish => { tx . close_now () ; } } } # [inline (always)] pub (crate) fn register_callback < CODEC > (spawner : & mut WorkerSpawner < ReactorWorker < R > , CODEC > ,) -> UnboundedReceiver < < R :: Scope as ReactorScoped > :: Output > where CODEC : Codec , { let (tx , rx) = mpsc :: unbounded () ; spawner . callback (move | output | Self :: output_callback (& tx , output)) ; rx } # [doc = " Forks the bridge."] # [doc = ""] # [doc = " This method creates a new bridge connected to a new reactor on the same worker instance."] pub fn fork (& self) -> Self { let (tx , rx) = mpsc :: unbounded () ; let inner = self . inner . fork (Some (move | output | Self :: output_callback (& tx , output))) ; Self { inner , rx } } # [doc = " Sends an input to the current reactor."] pub fn send_input (& self , msg : < R :: Scope as ReactorScoped > :: Input) { self . inner . send (ReactorInput :: Input (msg)) ; } }
};
}
