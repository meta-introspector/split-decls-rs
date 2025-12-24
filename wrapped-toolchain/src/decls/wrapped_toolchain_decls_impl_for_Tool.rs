use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Tool {
    pub fn proxy(self) -> Option<Utf8PathBuf> {
        cargo_proxy(self.name())
    }
    /// Return a `PathBuf` to use for the given executable.
    ///
    /// The current implementation checks three places for an executable to use:
    /// 1) `$CARGO_HOME/bin/<executable_name>`
    ///    where $CARGO_HOME defaults to ~/.cargo (see <https://doc.rust-lang.org/cargo/guide/cargo-home.html>)
    ///    example: for cargo, this tries $CARGO_HOME/bin/cargo, or ~/.cargo/bin/cargo if $CARGO_HOME is unset.
    ///    It seems that this is a reasonable place to try for cargo, rustc, and rustup
    /// 2) Appropriate environment variable (erroring if this is set but not a usable executable)
    ///    example: for cargo, this checks $CARGO environment variable; for rustc, $RUSTC; etc
    /// 3) $PATH/`<executable_name>`
    ///    example: for cargo, this tries all paths in $PATH with appended `cargo`, returning the
    ///    first that exists
    /// 4) If all else fails, we just try to use the executable name directly
    pub fn prefer_proxy(self) -> Utf8PathBuf {
        invoke(&[cargo_proxy, lookup_as_env_var, lookup_in_path], self.name())
    }
    /// Return a `PathBuf` to use for the given executable.
    ///
    /// The current implementation checks three places for an executable to use:
    /// 1) Appropriate environment variable (erroring if this is set but not a usable executable)
    ///    example: for cargo, this checks $CARGO environment variable; for rustc, $RUSTC; etc
    /// 2) $PATH/`<executable_name>`
    ///    example: for cargo, this tries all paths in $PATH with appended `cargo`, returning the
    ///    first that exists
    /// 3) `$CARGO_HOME/bin/<executable_name>`
    ///    where $CARGO_HOME defaults to ~/.cargo (see <https://doc.rust-lang.org/cargo/guide/cargo-home.html>)
    ///    example: for cargo, this tries $CARGO_HOME/bin/cargo, or ~/.cargo/bin/cargo if $CARGO_HOME is unset.
    ///    It seems that this is a reasonable place to try for cargo, rustc, and rustup
    /// 4) If all else fails, we just try to use the executable name directly
    pub fn path(self) -> Utf8PathBuf {
        invoke(&[lookup_as_env_var, lookup_in_path, cargo_proxy], self.name())
    }
    pub fn path_in(self, path: &Utf8Path) -> Option<Utf8PathBuf> {
        probe_for_binary(path.join(self.name()))
    }
    pub fn name(self) -> &'static str {
        match self {
            Tool::Cargo => "cargo",
            Tool::Rustc => "rustc",
            Tool::Rustup => "rustup",
            Tool::Rustfmt => "rustfmt",
        }
    }
}
