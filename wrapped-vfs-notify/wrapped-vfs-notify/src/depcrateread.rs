// Generated macro for read (function)
macro_rules! Depcrateread {
() => {
// Module: crate
// Provides: {"read"}
// Dependencies: {}
fn read (path : & AbsPath) -> Option < Vec < u8 > > { std :: fs :: read (path) . ok () }
};
}
