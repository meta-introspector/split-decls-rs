macro_rules! deps {
    () => {
        InlineAsmArch!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl FromStr for InlineAsmArch { type Err = () ; fn from_str (s : & str) -> Result < InlineAsmArch , () > { match s { "x86" => Ok (Self :: X86) , "x86_64" => Ok (Self :: X86_64) , "arm" => Ok (Self :: Arm) , "aarch64" => Ok (Self :: AArch64) , "arm64ec" => Ok (Self :: Arm64EC) , "riscv32" => Ok (Self :: RiscV32) , "riscv64" => Ok (Self :: RiscV64) , "nvptx64" => Ok (Self :: Nvptx64) , "powerpc" => Ok (Self :: PowerPC) , "powerpc64" => Ok (Self :: PowerPC64) , "hexagon" => Ok (Self :: Hexagon) , "loongarch32" => Ok (Self :: LoongArch32) , "loongarch64" => Ok (Self :: LoongArch64) , "mips" | "mips32r6" => Ok (Self :: Mips) , "mips64" | "mips64r6" => Ok (Self :: Mips64) , "s390x" => Ok (Self :: S390x) , "sparc" => Ok (Self :: Sparc) , "sparc64" => Ok (Self :: Sparc64) , "spirv" => Ok (Self :: SpirV) , "wasm32" => Ok (Self :: Wasm32) , "wasm64" => Ok (Self :: Wasm64) , "bpf" => Ok (Self :: Bpf) , "avr" => Ok (Self :: Avr) , "msp430" => Ok (Self :: Msp430) , "m68k" => Ok (Self :: M68k) , "csky" => Ok (Self :: CSKY) , _ => Err (()) , } } }
    };
}

impl_111!()