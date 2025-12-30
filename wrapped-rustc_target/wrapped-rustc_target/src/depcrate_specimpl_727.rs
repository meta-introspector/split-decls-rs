// Generated macro for impl_727 (impl)
macro_rules! Depcrate_specimpl_727 {
() => {
// Module: crate::spec
// Provides: {"impl_727"}
// Dependencies: {}
impl FromStr for LinkSelfContainedComponents { type Err = String ; # [doc = " Parses a single `-Clink-self-contained` well-known component, not a set of flags."] fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (match s { "crto" => LinkSelfContainedComponents :: CRT_OBJECTS , "libc" => LinkSelfContainedComponents :: LIBC , "unwind" => LinkSelfContainedComponents :: UNWIND , "linker" => LinkSelfContainedComponents :: LINKER , "sanitizers" => LinkSelfContainedComponents :: SANITIZERS , "mingw" => LinkSelfContainedComponents :: MINGW , _ => { return Err (format ! ("'{s}' is not a valid link-self-contained component, expected 'crto', 'libc', 'unwind', 'linker', 'sanitizers', 'mingw'")) ; } }) } }
};
}
