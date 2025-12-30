// Generated macro for install_client (function)
macro_rules! Depcrate_installinstall_client {
() => {
// Module: crate::install
// Provides: {"install_client"}
// Dependencies: {}
fn install_client (sh : & Shell , client_opt : ClientOpt) -> anyhow :: Result < () > { let _dir = sh . push_dir ("./editors/code") ; if cfg ! (unix) { cmd ! (sh , "npm --version") . run () . context ("`npm` is required to build the VS Code plugin") ? ; cmd ! (sh , "npm ci") . run () ? ; cmd ! (sh , "npm run package --scripts-prepend-node-path") . run () ? ; } else { cmd ! (sh , "cmd.exe /c npm --version") . run () . context ("`npm` is required to build the VS Code plugin") ? ; cmd ! (sh , "cmd.exe /c npm ci") . run () ? ; cmd ! (sh , "cmd.exe /c npm run package") . run () ? ; } ; let lifetime_extender ; let candidates : & [& str] = match client_opt . code_bin . as_deref () { Some (it) => { lifetime_extender = [it] ; & lifetime_extender [..] } None => VS_CODES , } ; let code = candidates . iter () . copied () . find (| & bin | { if cfg ! (unix) { cmd ! (sh , "{bin} --version") . read () . is_ok () } else { cmd ! (sh , "cmd.exe /c {bin}.cmd --version") . read () . is_ok () } }) . ok_or_else (| | { format_err ! ("Can't execute `{} --version`. Perhaps it is not in $PATH?" , candidates [0]) }) ? ; let installed_extensions = if cfg ! (unix) { cmd ! (sh , "{code} --install-extension rust-analyzer.vsix --force") . run () ? ; cmd ! (sh , "{code} --list-extensions") . read () ? } else { cmd ! (sh , "cmd.exe /c {code}.cmd --install-extension rust-analyzer.vsix --force") . run () ? ; cmd ! (sh , "cmd.exe /c {code}.cmd --list-extensions") . read () ? } ; if ! installed_extensions . contains ("rust-analyzer") { bail ! ("Could not install the Visual Studio Code extension. \
            Please make sure you have at least NodeJS 16.x together with the latest version of VS Code installed and try again. \
            Note that installing via xtask install does not work for VS Code Remote, instead you’ll need to install the .vsix manually.") ; } Ok (()) }
};
}
