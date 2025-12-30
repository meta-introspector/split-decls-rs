// Generated macro for impl_132 (impl)
macro_rules! Depcrateimpl_132 {
() => {
// Module: crate
// Provides: {"impl_132"}
// Dependencies: {}
impl Language { const ALL : & [Language] = & [Language :: Rust , Language :: C , Language :: Cpp , Language :: Wat , Language :: Csharp , Language :: MoonBit ,] ; fn obj (& self) -> & dyn LanguageMethods { match self { Language :: Rust => & rust :: Rust , Language :: C => & c :: C , Language :: Cpp => & cpp :: Cpp , Language :: Wat => & wat :: Wat , Language :: Csharp => & csharp :: Csharp , Language :: MoonBit => & moonbit :: MoonBit , Language :: Custom (custom) => custom , } } }
};
}
