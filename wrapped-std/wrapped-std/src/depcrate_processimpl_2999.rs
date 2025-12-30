// Generated macro for impl_2999 (impl)
macro_rules! Depcrate_processimpl_2999 {
() => {
// Module: crate::process
// Provides: {"impl_2999"}
// Dependencies: {}
# [stable (feature = "stdio_from" , since = "1.20.0")] impl From < fs :: File > for Stdio { # [doc = " Converts a [`File`](fs::File) into a [`Stdio`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " `File` will be converted to `Stdio` using `Stdio::from` under the hood."] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " use std::fs::File;"] # [doc = " use std::process::Command;"] # [doc = ""] # [doc = " // With the `foo.txt` file containing \"Hello, world!\""] # [doc = " let file = File::open(\"foo.txt\")?;"] # [doc = ""] # [doc = " let reverse = Command::new(\"rev\")"] # [doc = "     .stdin(file)  // Implicit File conversion into a Stdio"] # [doc = "     .output()?;"] # [doc = ""] # [doc = " assert_eq!(reverse.stdout, b\"!dlrow ,olleH\");"] # [doc = " # std::io::Result::Ok(())"] # [doc = " ```"] fn from (file : fs :: File) -> Stdio { Stdio :: from_inner (file . into_inner () . into ()) } }
};
}
