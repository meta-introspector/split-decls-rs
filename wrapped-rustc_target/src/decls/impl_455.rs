macro_rules! deps {
    () => {
        LinkerFlavorCli!();
        Lld!();
    };
}

macro_rules! impl_455 {
    () => {
        deps!();
        impl LinkerFlavorCli { # [doc = " Returns whether this `-C linker-flavor` option is one of the unstable values."] pub fn is_unstable (& self) -> bool { match self { LinkerFlavorCli :: Gnu (..) | LinkerFlavorCli :: Darwin (..) | LinkerFlavorCli :: WasmLld (..) | LinkerFlavorCli :: Unix (..) | LinkerFlavorCli :: Msvc (Lld :: Yes) | LinkerFlavorCli :: EmCc | LinkerFlavorCli :: Bpf | LinkerFlavorCli :: Llbc | LinkerFlavorCli :: Ptx => true , LinkerFlavorCli :: Gcc | LinkerFlavorCli :: Ld | LinkerFlavorCli :: Lld (..) | LinkerFlavorCli :: Msvc (Lld :: No) | LinkerFlavorCli :: Em => false , } } }
    };
}

impl_455!();