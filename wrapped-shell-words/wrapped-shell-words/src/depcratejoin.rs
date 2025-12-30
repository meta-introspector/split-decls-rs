// Generated macro for join (function)
macro_rules! Depcratejoin {
() => {
// Module: crate
// Provides: {"join"}
// Dependencies: {}
# [doc = " Joins arguments into a single command line suitable for execution in Unix"] # [doc = " shell."] # [doc = ""] # [doc = " Each argument is quoted using [`quote`] to preserve its literal meaning when"] # [doc = " parsed by Unix shell."] # [doc = ""] # [doc = " Note: This function is essentially an inverse of [`split`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Logging executed commands in format that can be easily copied and pasted"] # [doc = " into an actual shell:"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " fn execute(args: &[&str]) {"] # [doc = "     use std::process::Command;"] # [doc = "     println!(\"Executing: {}\", shell_words::join(args));"] # [doc = "     Command::new(&args[0])"] # [doc = "         .args(&args[1..])"] # [doc = "         .spawn()"] # [doc = "         .expect(\"failed to start subprocess\")"] # [doc = "         .wait()"] # [doc = "         .expect(\"failed to wait for subprocess\");"] # [doc = " }"] # [doc = ""] # [doc = " execute(&[\"python\", \"-c\", \"print('Hello world!')\"]);"] # [doc = " ```"] # [doc = ""] # [doc = " [`quote`]: fn.quote.html"] # [doc = " [`split`]: fn.split.html"] pub fn join < I , S > (words : I) -> String where I : IntoIterator < Item = S > , S : AsRef < str > , { let mut line = words . into_iter () . fold (String :: new () , | mut line , word | { let quoted = quote (word . as_ref ()) ; line . push_str (quoted . as_ref ()) ; line . push (' ') ; line }) ; line . pop () ; line }
};
}
