macro_rules! deps {
    () => {
        Abi!();
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_215 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_abi :: ExternAbi { type T = crate :: ty :: Abi ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use rustc_abi :: ExternAbi ; use crate :: ty :: Abi ; match * self { ExternAbi :: Rust => Abi :: Rust , ExternAbi :: C { unwind } => Abi :: C { unwind } , ExternAbi :: Cdecl { unwind } => Abi :: Cdecl { unwind } , ExternAbi :: Stdcall { unwind } => Abi :: Stdcall { unwind } , ExternAbi :: Fastcall { unwind } => Abi :: Fastcall { unwind } , ExternAbi :: Vectorcall { unwind } => Abi :: Vectorcall { unwind } , ExternAbi :: Thiscall { unwind } => Abi :: Thiscall { unwind } , ExternAbi :: Aapcs { unwind } => Abi :: Aapcs { unwind } , ExternAbi :: Win64 { unwind } => Abi :: Win64 { unwind } , ExternAbi :: SysV64 { unwind } => Abi :: SysV64 { unwind } , ExternAbi :: PtxKernel => Abi :: PtxKernel , ExternAbi :: GpuKernel => Abi :: GpuKernel , ExternAbi :: Msp430Interrupt => Abi :: Msp430Interrupt , ExternAbi :: X86Interrupt => Abi :: X86Interrupt , ExternAbi :: EfiApi => Abi :: EfiApi , ExternAbi :: AvrInterrupt => Abi :: AvrInterrupt , ExternAbi :: AvrNonBlockingInterrupt => Abi :: AvrNonBlockingInterrupt , ExternAbi :: CmseNonSecureCall => Abi :: CCmseNonSecureCall , ExternAbi :: CmseNonSecureEntry => Abi :: CCmseNonSecureEntry , ExternAbi :: System { unwind } => Abi :: System { unwind } , ExternAbi :: RustCall => Abi :: RustCall , ExternAbi :: Unadjusted => Abi :: Unadjusted , ExternAbi :: RustCold => Abi :: RustCold , ExternAbi :: RustInvalid => Abi :: RustInvalid , ExternAbi :: RiscvInterruptM => Abi :: RiscvInterruptM , ExternAbi :: RiscvInterruptS => Abi :: RiscvInterruptS , ExternAbi :: Custom => Abi :: Custom , } } }
    };
}

impl_215!();