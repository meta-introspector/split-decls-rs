// Generated macro for Compile (struct)
macro_rules! DepcrateCompile {
() => {
// Module: crate
// Provides: {"Compile"}
// Dependencies: {}
# [doc = " Helper structure to package up arguments when sent to language-specific"] # [doc = " compilation backends for `LanguageMethods::compile`"] struct Compile < 'a > { component : & 'a Component , bindings_dir : & 'a Path , artifacts_dir : & 'a Path , output : & 'a Path , }
};
}
