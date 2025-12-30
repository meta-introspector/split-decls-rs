// Generated macro for reformat (function)
macro_rules! Depcrate_codegenreformat {
() => {
// Module: crate::codegen
// Provides: {"reformat"}
// Dependencies: {}
fn reformat (text : String) -> String { let sh = Shell :: new () . unwrap () ; let rustfmt_toml = project_root () . join ("rustfmt.toml") ; let version = cmd ! (sh , "rustup run stable rustfmt --version") . read () . unwrap_or_default () ; let mut stdout = if ! version . contains ("stable") { let version = cmd ! (sh , "rustfmt --version") . read () . unwrap_or_default () ; if ! version . contains ("stable") { panic ! ("Failed to run rustfmt from toolchain 'stable'. \
                 Please run `rustup component add rustfmt --toolchain stable` to install it." ,) ; } else { cmd ! (sh , "rustfmt --config-path {rustfmt_toml} --config fn_single_line=true") . stdin (text) . read () . unwrap () } } else { cmd ! (sh , "rustup run stable rustfmt --config-path {rustfmt_toml} --config fn_single_line=true") . stdin (text) . read () . unwrap () } ; if ! stdout . ends_with ('\n') { stdout . push ('\n') ; } stdout }
};
}
