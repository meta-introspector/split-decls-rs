// Generated macro for impl_112 (impl)
macro_rules! Depcrateimpl_112 {
() => {
// Module: crate
// Provides: {"impl_112"}
// Dependencies: {}
impl SendQueue { fn new (encoded_labels : String) -> SendQueue { SendQueue { encoded_labels , sending : Vec :: new () , to_send : Vec :: new () , } } fn push (& mut self , event : LokiEvent) { self . to_send . push (event) ; } fn drop_outstanding (& mut self) -> usize { let len = self . sending . len () ; self . sending . clear () ; len } fn on_send_result (& mut self , result : Result < () , () >) { match result { Ok (()) => self . sending . clear () , Err (()) => { self . sending . append (& mut self . to_send) ; mem :: swap (& mut self . sending , & mut self . to_send) ; } } } fn should_send (& self) -> bool { self . to_send . iter () . any (| e | e . trigger_send) } fn prepare_sending (& mut self) -> loki :: StreamAdapter { if ! self . sending . is_empty () { panic ! ("can only prepare sending while no request is in flight") ; } mem :: swap (& mut self . sending , & mut self . to_send) ; loki :: StreamAdapter { labels : self . encoded_labels . clone () , entries : self . sending . iter () . map (| e | loki :: EntryAdapter { timestamp : Some (e . timestamp . into ()) , line : e . message . clone () , }) . collect () , hash : 0 , } } }
};
}
