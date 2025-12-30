// Generated macro for Console (struct)
macro_rules! Depcrate_consoleConsole {
() => {
// Module: crate::console
// Provides: {"Console"}
// Dependencies: {}
# [doc = " A Windows console."] # [doc = ""] # [doc = " This represents a very limited set of functionality available to a Windows"] # [doc = " console. In particular, it can only change text attributes such as color"] # [doc = " and intensity. This may grow over time. If you need more routines, please"] # [doc = " file an issue and/or PR."] # [doc = ""] # [doc = " There is no way to \"write\" to this console. Simply write to"] # [doc = " stdout or stderr instead, while interleaving instructions to the console"] # [doc = " to change text attributes."] # [doc = ""] # [doc = " A common pitfall when using a console is to forget to flush writes to"] # [doc = " stdout before setting new text attributes."] # [doc = ""] # [doc = " # Example"] # [doc = " ```no_run"] # [doc = " # #[cfg(windows)]"] # [doc = " # {"] # [doc = " use winapi_util::console::{Console, Color, Intense};"] # [doc = ""] # [doc = " let mut con = Console::stdout().unwrap();"] # [doc = " con.fg(Intense::Yes, Color::Cyan).unwrap();"] # [doc = " println!(\"This text will be intense cyan.\");"] # [doc = " con.reset().unwrap();"] # [doc = " println!(\"This text will be normal.\");"] # [doc = " # }"] # [doc = " ```"] # [derive (Debug)] pub struct Console { kind : HandleKind , start_attr : TextAttributes , cur_attr : TextAttributes , }
};
}
