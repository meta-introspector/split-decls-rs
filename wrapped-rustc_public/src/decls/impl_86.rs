macro_rules! deps {
    () => {
        RustcInternal!();
        BridgeTys!();
        InternalCx!();
        Abi!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl RustcInternal for Abi { type T < 'tcx > = rustc_abi :: ExternAbi ; fn internal < 'tcx > (& self , _tables : & mut Tables < '_ , BridgeTys > , _tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { match * self { Abi :: Rust => rustc_abi :: ExternAbi :: Rust , Abi :: C { unwind } => rustc_abi :: ExternAbi :: C { unwind } , Abi :: Cdecl { unwind } => rustc_abi :: ExternAbi :: Cdecl { unwind } , Abi :: Stdcall { unwind } => rustc_abi :: ExternAbi :: Stdcall { unwind } , Abi :: Fastcall { unwind } => rustc_abi :: ExternAbi :: Fastcall { unwind } , Abi :: Vectorcall { unwind } => rustc_abi :: ExternAbi :: Vectorcall { unwind } , Abi :: Thiscall { unwind } => rustc_abi :: ExternAbi :: Thiscall { unwind } , Abi :: Aapcs { unwind } => rustc_abi :: ExternAbi :: Aapcs { unwind } , Abi :: CCmseNonSecureCall => rustc_abi :: ExternAbi :: CmseNonSecureCall , Abi :: CCmseNonSecureEntry => rustc_abi :: ExternAbi :: CmseNonSecureEntry , Abi :: Win64 { unwind } => rustc_abi :: ExternAbi :: Win64 { unwind } , Abi :: SysV64 { unwind } => rustc_abi :: ExternAbi :: SysV64 { unwind } , Abi :: PtxKernel => rustc_abi :: ExternAbi :: PtxKernel , Abi :: Msp430Interrupt => rustc_abi :: ExternAbi :: Msp430Interrupt , Abi :: X86Interrupt => rustc_abi :: ExternAbi :: X86Interrupt , Abi :: GpuKernel => rustc_abi :: ExternAbi :: GpuKernel , Abi :: EfiApi => rustc_abi :: ExternAbi :: EfiApi , Abi :: AvrInterrupt => rustc_abi :: ExternAbi :: AvrInterrupt , Abi :: AvrNonBlockingInterrupt => rustc_abi :: ExternAbi :: AvrNonBlockingInterrupt , Abi :: System { unwind } => rustc_abi :: ExternAbi :: System { unwind } , Abi :: RustCall => rustc_abi :: ExternAbi :: RustCall , Abi :: Unadjusted => rustc_abi :: ExternAbi :: Unadjusted , Abi :: RustCold => rustc_abi :: ExternAbi :: RustCold , Abi :: RustInvalid => rustc_abi :: ExternAbi :: RustInvalid , Abi :: RiscvInterruptM => rustc_abi :: ExternAbi :: RiscvInterruptM , Abi :: RiscvInterruptS => rustc_abi :: ExternAbi :: RiscvInterruptS , Abi :: Custom => rustc_abi :: ExternAbi :: Custom , } } }
    };
}

impl_86!()