macro_rules! deps {
    () => {
        LinkerFlavorCli!();
        Lld!();
        Cc!();
    };
}

macro_rules! macro_459 {
    () => {
        deps!();
        linker_flavor_cli_impls ! { (LinkerFlavorCli :: Gnu (Cc :: No , Lld :: No)) "gnu" (LinkerFlavorCli :: Gnu (Cc :: No , Lld :: Yes)) "gnu-lld" (LinkerFlavorCli :: Gnu (Cc :: Yes , Lld :: No)) "gnu-cc" (LinkerFlavorCli :: Gnu (Cc :: Yes , Lld :: Yes)) "gnu-lld-cc" (LinkerFlavorCli :: Darwin (Cc :: No , Lld :: No)) "darwin" (LinkerFlavorCli :: Darwin (Cc :: No , Lld :: Yes)) "darwin-lld" (LinkerFlavorCli :: Darwin (Cc :: Yes , Lld :: No)) "darwin-cc" (LinkerFlavorCli :: Darwin (Cc :: Yes , Lld :: Yes)) "darwin-lld-cc" (LinkerFlavorCli :: WasmLld (Cc :: No)) "wasm-lld" (LinkerFlavorCli :: WasmLld (Cc :: Yes)) "wasm-lld-cc" (LinkerFlavorCli :: Unix (Cc :: No)) "unix" (LinkerFlavorCli :: Unix (Cc :: Yes)) "unix-cc" (LinkerFlavorCli :: Msvc (Lld :: Yes)) "msvc-lld" (LinkerFlavorCli :: Msvc (Lld :: No)) "msvc" (LinkerFlavorCli :: EmCc) "em-cc" (LinkerFlavorCli :: Bpf) "bpf" (LinkerFlavorCli :: Llbc) "llbc" (LinkerFlavorCli :: Ptx) "ptx" (LinkerFlavorCli :: Gcc) "gcc" (LinkerFlavorCli :: Ld) "ld" (LinkerFlavorCli :: Lld (LldFlavor :: Ld)) "ld.lld" (LinkerFlavorCli :: Lld (LldFlavor :: Ld64)) "ld64.lld" (LinkerFlavorCli :: Lld (LldFlavor :: Link)) "lld-link" (LinkerFlavorCli :: Lld (LldFlavor :: Wasm)) "wasm-ld" (LinkerFlavorCli :: Em) "em" }
    };
}

macro_459!()