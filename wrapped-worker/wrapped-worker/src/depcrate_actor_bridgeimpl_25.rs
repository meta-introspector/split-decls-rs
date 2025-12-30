// Generated macro for impl_25 (impl)
macro_rules! Depcrate_actor_bridgeimpl_25 {
() => {
// Module: crate::actor::bridge
// Provides: {"impl_25"}
// Dependencies: {}
impl < W > WorkerBridge < W > where W : Worker , { fn init (& self) { self . inner . send_message (ToWorker :: Connected (self . id)) ; } pub (crate) fn new < CODEC > (id : HandlerId , native_worker : web_sys :: Worker , pending_queue : Rc < RefCell < Option < ToWorkerQueue < W > > > > , callbacks : Rc < RefCell < CallbackMap < W > > > , callback : Option < Callback < W :: Output > > ,) -> Self where CODEC : Codec , W :: Input : Serialize + for < 'de > Deserialize < 'de > , { let post_msg = move | msg : ToWorker < W > | native_worker . post_packed_message :: < _ , CODEC > (msg) ; let self_ = Self { inner : WorkerBridgeInner { pending_queue , callbacks , post_msg : Rc :: new (post_msg) , } . into () , id , _worker : PhantomData , _cb : callback , } ; self_ . init () ; self_ } # [doc = " Send a message to the current worker."] pub fn send (& self , msg : W :: Input) { let msg = ToWorker :: ProcessInput (self . id , msg) ; self . inner . send_message (msg) ; } # [doc = " Forks the bridge with a different callback."] # [doc = ""] # [doc = " This creates a new [HandlerID] that helps the worker to differentiate bridges."] pub fn fork < F > (& self , cb : Option < F >) -> Self where F : 'static + Fn (W :: Output) , { let cb = cb . map (| m | Rc :: new (m) as Rc < dyn Fn (W :: Output) >) ; let handler_id = HandlerId :: new () ; if let Some (cb_weak) = cb . as_ref () . map (Rc :: downgrade) { self . inner . callbacks . borrow_mut () . insert (handler_id , cb_weak) ; } let self_ = Self { inner : self . inner . clone () , id : handler_id , _worker : PhantomData , _cb : cb , } ; self_ . init () ; self_ } }
};
}
