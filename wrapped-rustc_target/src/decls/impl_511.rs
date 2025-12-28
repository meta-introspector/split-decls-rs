macro_rules! deps {
    () => {
        SanitizerSet!();
    };
}

macro_rules! impl_511 {
    () => {
        deps!();
        impl FromStr for SanitizerSet { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (match s { "address" => SanitizerSet :: ADDRESS , "cfi" => SanitizerSet :: CFI , "dataflow" => SanitizerSet :: DATAFLOW , "kcfi" => SanitizerSet :: KCFI , "kernel-address" => SanitizerSet :: KERNELADDRESS , "leak" => SanitizerSet :: LEAK , "memory" => SanitizerSet :: MEMORY , "memtag" => SanitizerSet :: MEMTAG , "safestack" => SanitizerSet :: SAFESTACK , "shadow-call-stack" => SanitizerSet :: SHADOWCALLSTACK , "thread" => SanitizerSet :: THREAD , "hwaddress" => SanitizerSet :: HWADDRESS , s => return Err (format ! ("unknown sanitizer {s}")) , }) } }
    };
}

impl_511!()