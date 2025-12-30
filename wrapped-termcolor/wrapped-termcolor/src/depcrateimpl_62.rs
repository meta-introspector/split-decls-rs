// Generated macro for impl_62 (impl)
macro_rules! Depcrateimpl_62 {
() => {
// Module: crate
// Provides: {"impl_62"}
// Dependencies: {}
# [cfg (windows)] impl WindowsBuffer { # [doc = " Create a new empty buffer for Windows console coloring."] fn new () -> WindowsBuffer { WindowsBuffer { buf : vec ! [] , colors : vec ! [] } } # [doc = " Push the given color specification into this buffer."] # [doc = ""] # [doc = " This has the effect of setting the given color information at the"] # [doc = " current position in the buffer."] fn push (& mut self , spec : Option < ColorSpec >) { let pos = self . buf . len () ; self . colors . push ((pos , spec)) ; } # [doc = " Print the contents to the given stream handle, and use the console"] # [doc = " for coloring."] fn print (& self , console : & mut wincon :: Console , stream : & mut LossyStandardStream < IoStandardStreamLock > ,) -> io :: Result < () > { let mut last = 0 ; for & (pos , ref spec) in & self . colors { stream . write_all (& self . buf [last .. pos]) ? ; stream . flush () ? ; last = pos ; match * spec { None => console . reset () ? , Some (ref spec) => spec . write_console (console) ? , } } stream . write_all (& self . buf [last ..]) ? ; stream . flush () } # [doc = " Clear the buffer."] fn clear (& mut self) { self . buf . clear () ; self . colors . clear () ; } }
};
}
