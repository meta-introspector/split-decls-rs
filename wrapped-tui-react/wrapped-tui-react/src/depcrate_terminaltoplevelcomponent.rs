// Generated macro for ToplevelComponent (trait)
macro_rules! Depcrate_terminalToplevelComponent {
() => {
// Module: crate::terminal
// Provides: {"ToplevelComponent"}
// Dependencies: {}
# [doc = " A component meant to be rendered by `Terminal::render(...)`."] # [doc = " All other components don't have to implement this trait, and instead"] # [doc = " provide a render method by convention, tuned towards their needs using whichever"] # [doc = " generic types or lifetimes they need."] pub trait ToplevelComponent { type Props ; fn render (& mut self , props : impl Borrow < Self :: Props > , area : Rect , buf : & mut Buffer) ; }
};
}
