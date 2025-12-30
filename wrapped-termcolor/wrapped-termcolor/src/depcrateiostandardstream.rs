// Generated macro for IoStandardStream (enum)
macro_rules! DepcrateIoStandardStream {
() => {
// Module: crate
// Provides: {"IoStandardStream"}
// Dependencies: {}
# [derive (Debug)] enum IoStandardStream { Stdout (io :: Stdout) , Stderr (io :: Stderr) , StdoutBuffered (io :: BufWriter < io :: Stdout >) , StderrBuffered (io :: BufWriter < io :: Stderr >) , }
};
}
