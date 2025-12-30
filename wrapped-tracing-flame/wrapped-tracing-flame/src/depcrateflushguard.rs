// Generated macro for FlushGuard (struct)
macro_rules! DepcrateFlushGuard {
() => {
// Module: crate
// Provides: {"FlushGuard"}
// Dependencies: {}
# [doc = " An RAII guard for managing flushing a global writer that is"] # [doc = " otherwise inaccessible."] # [doc = ""] # [doc = " This type is only needed when using"] # [doc = " `tracing::subscriber::set_global_default`, which prevents the drop"] # [doc = " implementation of layers from running when the program exits."] # [must_use] # [derive (Debug)] pub struct FlushGuard < W > where W : Write + 'static , { out : Arc < Mutex < W > > , }
};
}
