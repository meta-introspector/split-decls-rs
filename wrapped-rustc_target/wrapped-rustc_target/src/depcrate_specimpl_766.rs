// Generated macro for impl_766 (impl)
macro_rules! Depcrate_specimpl_766 {
() => {
// Module: crate::spec
// Provides: {"impl_766"}
// Dependencies: {}
impl FromStr for SanitizerSet { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (match s { "address" => SanitizerSet :: ADDRESS , "cfi" => SanitizerSet :: CFI , "dataflow" => SanitizerSet :: DATAFLOW , "kcfi" => SanitizerSet :: KCFI , "kernel-address" => SanitizerSet :: KERNELADDRESS , "leak" => SanitizerSet :: LEAK , "memory" => SanitizerSet :: MEMORY , "memtag" => SanitizerSet :: MEMTAG , "safestack" => SanitizerSet :: SAFESTACK , "shadow-call-stack" => SanitizerSet :: SHADOWCALLSTACK , "thread" => SanitizerSet :: THREAD , "hwaddress" => SanitizerSet :: HWADDRESS , s => return Err (format ! ("unknown sanitizer {s}")) , }) } }
};
}
