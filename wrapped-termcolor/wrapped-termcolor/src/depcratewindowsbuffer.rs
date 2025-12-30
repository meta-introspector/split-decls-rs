// Generated macro for WindowsBuffer (struct)
macro_rules! DepcrateWindowsBuffer {
() => {
// Module: crate
// Provides: {"WindowsBuffer"}
// Dependencies: {}
# [doc = " An in-memory buffer that provides Windows console coloring."] # [doc = ""] # [doc = " This doesn't actually communicate with the Windows console. Instead, it"] # [doc = " acts like a normal buffer but also saves the color information associated"] # [doc = " with positions in the buffer. It is only when the buffer is written to the"] # [doc = " console that coloring is actually applied."] # [doc = ""] # [doc = " This is roughly isomorphic to the ANSI based approach (i.e.,"] # [doc = " `Ansi<Vec<u8>>`), except with ANSI, the color information is embedded"] # [doc = " directly into the buffer."] # [doc = ""] # [doc = " Note that there is no way to write something generic like"] # [doc = " `WindowsConsole<W: io::Write>` since coloring on Windows is tied"] # [doc = " specifically to the console APIs, and therefore can't work on arbitrary"] # [doc = " writers."] # [cfg (windows)] # [derive (Clone , Debug)] struct WindowsBuffer { # [doc = " The actual content that should be printed."] buf : Vec < u8 > , # [doc = " A sequence of position oriented color specifications. Namely, each"] # [doc = " element is a position and a color spec, where the color spec should"] # [doc = " be applied at the position inside of `buf`."] # [doc = ""] # [doc = " A missing color spec implies the underlying console should be reset."] colors : Vec < (usize , Option < ColorSpec >) > , }
};
}
