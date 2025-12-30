// Generated macro for Running (struct)
macro_rules! Depcrate_subscriberRunning {
() => {
// Module: crate::subscriber
// Provides: {"Running"}
// Dependencies: {}
struct Running < F : Fn (& Metadata < '_ >) -> bool > { spans : Mutex < HashMap < Id , SpanState > > , expected : Arc < Mutex < VecDeque < Expect > > > , current : Mutex < Vec < Id > > , ids : AtomicUsize , max_level : Option < LevelFilter > , filter : F , name : String , }
};
}
