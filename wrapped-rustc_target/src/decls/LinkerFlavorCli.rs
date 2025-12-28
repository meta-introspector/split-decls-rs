macro_rules! deps {
    () => {
        Cc!();
        Lld!();
        LinkerFlavor!();
    };
}

macro_rules! LinkerFlavorCli {
    () => {
        deps!();
        # [doc = " Linker flavors available externally through command line (`-Clinker-flavor`)"] # [doc = " or json target specifications."] # [doc = " This set has accumulated historically, and contains both (stable and unstable) legacy values, as"] # [doc = " well as modern ones matching the internal linker flavors (`LinkerFlavor`)."] # [derive (Clone , Copy , Debug , Eq , Ord , PartialEq , PartialOrd)] pub enum LinkerFlavorCli { Gnu (Cc , Lld) , Darwin (Cc , Lld) , WasmLld (Cc) , Unix (Cc) , Msvc (Lld) , EmCc , Bpf , Ptx , Llbc , Gcc , Ld , Lld (LldFlavor) , Em , }
    };
}

LinkerFlavorCli!()