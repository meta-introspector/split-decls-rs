// Generated macro for log (module)
macro_rules! Depcratelog {
() => {
// Module: crate
// Provides: {"log"}
// Dependencies: {}
# [cfg (feature = "log")] # [doc (hidden)] pub mod log { use core :: fmt ; pub use log :: * ; use tracing_core :: field :: { Field , ValueSet , Visit } ; # [doc = " Utility to format [`ValueSet`]s for logging."] pub (crate) struct LogValueSet < 'a > { pub (crate) values : & 'a ValueSet < 'a > , pub (crate) is_first : bool , } impl < 'a > fmt :: Display for LogValueSet < 'a > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { struct LogVisitor < 'a , 'b > { f : & 'a mut fmt :: Formatter < 'b > , is_first : bool , result : fmt :: Result , } impl Visit for LogVisitor < '_ , '_ > { fn record_debug (& mut self , field : & Field , value : & dyn fmt :: Debug) { let res = if self . is_first { self . is_first = false ; if field . name () == "message" { write ! (self . f , "{:?}" , value) } else { write ! (self . f , "{}={:?}" , field . name () , value) } } else { write ! (self . f , " {}={:?}" , field . name () , value) } ; if let Err (err) = res { self . result = self . result . and (Err (err)) ; } } fn record_str (& mut self , field : & Field , value : & str) { if field . name () == "message" { self . record_debug (field , & format_args ! ("{}" , value)) } else { self . record_debug (field , & value) } } } let mut visit = LogVisitor { f , is_first : self . is_first , result : Ok (()) , } ; self . values . record (& mut visit) ; visit . result } } }
};
}
