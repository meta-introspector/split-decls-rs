// Generated macro for errors (module)
macro_rules! Depcrateerrors {
() => {
// Module: crate
// Provides: {"errors"}
// Dependencies: {}
mod errors { use std :: io ; use tungstenite ; use serde_json ; use futures :: Future ; error_chain ! { foreign_links { Ws (tungstenite :: Error) ; Io (io :: Error) ; Json (serde_json :: Error) ; } } pub type MyFuture < T > = Box < Future < Item = T , Error = Error > > ; }
};
}
