// Generated macro for push_toolchain (function)
macro_rules! Depcratepush_toolchain {
() => {
// Module: crate
// Provides: {"push_toolchain"}
// Dependencies: {}
fn push_toolchain < 'a > (sh : & 'a xshell :: Shell , toolchain : & str ,) -> xshell :: Result < xshell :: PushEnv < 'a > > { cmd ! (sh , "rustup toolchain install {toolchain} --no-self-update") . run () ? ; let res = sh . push_env ("RUSTUP_TOOLCHAIN" , toolchain) ; cmd ! (sh , "rustc --version") . run () ? ; Ok (res) }
};
}
