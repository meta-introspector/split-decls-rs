// Generated macro for impl_133 (impl)
macro_rules! Depcrate_docimpl_133 {
() => {
// Module: crate::doc
// Provides: {"impl_133"}
// Dependencies: {}
impl Doc { pub fn run (self) -> Result < () > { let sh = crate :: sh () ? ; for arch in Arch :: all () { arch . install () ? ; let triple = arch . triple () ; cmd ! (sh , "cargo doc --target={triple}") . arg ("--no-deps") . arg ("--document-private-items") . arg ("--package=hermit-kernel") . run () ? ; cmd ! (sh , "cargo doc --target={triple}") . arg ("--no-deps") . arg ("--document-private-items") . arg ("--manifest-path=hermit-builtins/Cargo.toml") . run () ? ; } cmd ! (sh , "cargo doc") . arg ("--no-deps") . arg ("--document-private-items") . arg ("--package=xtask") . run () ? ; Ok (()) } }
};
}
