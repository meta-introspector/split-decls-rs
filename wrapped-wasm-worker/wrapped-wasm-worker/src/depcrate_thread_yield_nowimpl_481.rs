// Generated macro for impl_481 (impl)
macro_rules! Depcrate_thread_yield_nowimpl_481 {
() => {
// Module: crate::thread::yield_now
// Provides: {"impl_481"}
// Dependencies: {}
impl WakerData { # [doc = " Creates a new [`WakerData`] and a corresponding [`Closure`]."] fn new () -> (Rc < RefCell < Self > > , Closure < dyn FnMut () >) { let this = Rc :: new (RefCell :: new (Self { completed : false , waker : None , })) ; let callback = Closure :: once ({ let this = Rc :: clone (& this) ; move | | this . borrow_mut () . complete () }) ; (this , callback) } # [doc = " Completes the [`Future`] and wakes up a registered [`Waker`]."] fn complete (& mut self) { self . completed = true ; if let Some (waker) = self . waker . take () { waker . wake () ; } } # [doc = " Polls the [`WakerData`]."] fn poll (& mut self , cx : & Context < '_ >) -> Poll < () > { if self . completed { Poll :: Ready (()) } else { self . waker = Some (cx . waker () . clone ()) ; Poll :: Pending } } }
};
}
