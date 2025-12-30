// Generated macro for impl_130 (impl)
macro_rules! Depcrate_thread_atomics_channelimpl_130 {
() => {
// Module: crate::thread::atomics::channel
// Provides: {"impl_130"}
// Dependencies: {}
impl < T > Receiver < T > { # [doc = " Attempts to return a pending value on this receiver without blocking."] # [cfg (feature = "message")] pub (super) fn try_recv (& self) -> Result < T , TryRecvError > { self . receiver . try_recv () } # [doc = " Wait for the next event sent by the [`Sender`]."] pub (super) async fn next (& self) -> Result < T , RecvError > { future :: poll_fn (| cx | match self . receiver . try_recv () { Ok (event) => Poll :: Ready (Ok (event)) , Err (TryRecvError :: Empty) => { self . waker . register (cx . waker ()) ; match self . receiver . try_recv () { Ok (event) => Poll :: Ready (Ok (event)) , Err (TryRecvError :: Empty) => Poll :: Pending , Err (TryRecvError :: Disconnected) => Poll :: Ready (Err (RecvError)) , } } Err (TryRecvError :: Disconnected) => Poll :: Ready (Err (RecvError)) , }) . await } }
};
}
