// Generated macro for Shell (struct)
macro_rules! DepcrateShell {
() => {
// Module: crate
// Provides: {"Shell"}
// Dependencies: {}
# [doc = " A `Shell` is the main API entry point."] # [doc = ""] # [doc = " Almost all of the crate's functionality is available as methods of the `Shell` object."] # [doc = ""] # [doc = " `Shell` is a stateful object. It maintains a logical working directory and an environment map."] # [doc = " They are independent from process's [`std::env::current_dir`] and [`std::env::var`], and only"] # [doc = " affect paths and commands passed to the [`Shell`]. `Shell` is cheaply clonable and you can use"] # [doc = " methods like [`Shell::with_current_dir`] to create independent copies with separate"] # [doc = " environments."] # [doc = ""] # [doc = " By convention, the variable holding the shell is named `sh`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use xshell::{cmd, Shell};"] # [doc = ""] # [doc = " let sh = Shell::new()?;"] # [doc = " let sh = sh.with_current_dir(\"./target\");"] # [doc = " let cwd = sh.current_dir();"] # [doc = " cmd!(sh, \"echo current dir is {cwd}\").run()?;"] # [doc = ""] # [doc = " let process_cwd = std::env::current_dir().unwrap();"] # [doc = " assert_eq!(cwd, process_cwd.join(\"./target\"));"] # [doc = " # Ok::<(), xshell::Error>(())"] # [doc = " ```"] # [derive (Debug , Clone)] pub struct Shell { cwd : Arc < Path > , env : Arc < HashMap < Arc < OsStr > , Arc < OsStr > > > , }
};
}
