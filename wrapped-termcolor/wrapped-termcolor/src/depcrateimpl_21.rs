// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl IoStandardStream { fn new (sty : StandardStreamType) -> IoStandardStream { match sty { StandardStreamType :: Stdout => { IoStandardStream :: Stdout (io :: stdout ()) } StandardStreamType :: Stderr => { IoStandardStream :: Stderr (io :: stderr ()) } StandardStreamType :: StdoutBuffered => { let wtr = io :: BufWriter :: new (io :: stdout ()) ; IoStandardStream :: StdoutBuffered (wtr) } StandardStreamType :: StderrBuffered => { let wtr = io :: BufWriter :: new (io :: stderr ()) ; IoStandardStream :: StderrBuffered (wtr) } } } fn lock (& self) -> IoStandardStreamLock < '_ > { match * self { IoStandardStream :: Stdout (ref s) => { IoStandardStreamLock :: StdoutLock (s . lock ()) } IoStandardStream :: Stderr (ref s) => { IoStandardStreamLock :: StderrLock (s . lock ()) } IoStandardStream :: StdoutBuffered (_) | IoStandardStream :: StderrBuffered (_) => { panic ! ("cannot lock a buffered standard stream") } } } }
};
}
