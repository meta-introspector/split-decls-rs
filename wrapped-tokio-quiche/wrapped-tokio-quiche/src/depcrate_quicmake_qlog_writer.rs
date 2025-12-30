// Generated macro for make_qlog_writer (function)
macro_rules! Depcrate_quicmake_qlog_writer {
() => {
// Module: crate::quic
// Provides: {"make_qlog_writer"}
// Dependencies: {}
fn make_qlog_writer (dir : & str , id : & str ,) -> std :: io :: Result < std :: io :: BufWriter < std :: fs :: File > > { let mut path = std :: path :: PathBuf :: from (dir) ; let filename = format ! ("{id}.sqlog") ; path . push (filename) ; let f = std :: fs :: File :: create (& path) ? ; Ok (std :: io :: BufWriter :: new (f)) }
};
}
