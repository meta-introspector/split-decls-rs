// Generated macro for Cmd (struct)
macro_rules! DepcrateCmd {
() => {
// Module: crate
// Provides: {"Cmd"}
// Dependencies: {}
# [doc = " A builder object for constructing a subprocess."] # [doc = ""] # [doc = " A [`Cmd`] is usually created with the [`cmd!`] macro. The command exists within a context of a"] # [doc = " [`Shell`] and uses its working directory and environment."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use xshell::{Shell, cmd};"] # [doc = ""] # [doc = " let sh = Shell::new()?;"] # [doc = ""] # [doc = " let branch = \"main\";"] # [doc = " let cmd = cmd!(sh, \"git switch {branch}\").run()?;"] # [doc = " # Ok::<(), xshell::Error>(())"] # [doc = " ```"] # [doc = ""] # [doc = " Use:"] # [doc = ""] # [doc = " * [`Cmd::run_echo`] for interactive scripts where the user watches the output live."] # [doc = " * [`Cmd::run`] for batch scripts where the output matters only if an error occurs."] # [doc = " * [`Cmd::read`] to get command's output."] # [doc = ""] # [doc = " Methods for fine-grained control over child process stdio are intentionally not provided. If you"] # [doc = " need anything not covered by `Cmd` API, use [`Cmd::to_command`] to convert it to"] # [doc = " [`std::process::Command`]."] # [derive (Debug , Clone)] # [must_use] pub struct Cmd { sh : Shell , prog : PathBuf , args : Vec < OsString > , stdin_contents : Option < Vec < u8 > > , deadline : Option < Instant > , ignore_status : bool , secret : bool , }
};
}
