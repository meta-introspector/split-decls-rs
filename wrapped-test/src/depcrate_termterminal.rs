// Generated macro for Terminal (trait)
macro_rules! Depcrate_termTerminal {
() => {
// Module: crate::term
// Provides: {"Terminal"}
// Dependencies: {}
# [doc = " A terminal with similar capabilities to an ANSI Terminal"] # [doc = " (foreground/background colors etc)."] pub (crate) trait Terminal : Write { # [doc = " Sets the foreground color to the given color."] # [doc = ""] # [doc = " If the color is a bright color, but the terminal only supports 8 colors,"] # [doc = " the corresponding normal color will be used instead."] # [doc = ""] # [doc = " Returns `Ok(true)` if the color was set, `Ok(false)` otherwise, and `Err(e)`"] # [doc = " if there was an I/O error."] fn fg (& mut self , color : color :: Color) -> io :: Result < bool > ; # [doc = " Resets all terminal attributes and colors to their defaults."] # [doc = ""] # [doc = " Returns `Ok(true)` if the terminal was reset, `Ok(false)` otherwise, and `Err(e)` if there"] # [doc = " was an I/O error."] # [doc = ""] # [doc = " *Note: This does not flush.*"] # [doc = ""] # [doc = " That means the reset command may get buffered so, if you aren't planning on doing anything"] # [doc = " else that might flush stdout's buffer (e.g., writing a line of text), you should flush after"] # [doc = " calling reset."] fn reset (& mut self) -> io :: Result < bool > ; }
};
}
