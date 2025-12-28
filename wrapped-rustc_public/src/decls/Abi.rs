macro_rules! Abi {
    () => {
        # [derive (Clone , PartialEq , Eq , Debug , Serialize)] pub enum Abi { Rust , C { unwind : bool } , Cdecl { unwind : bool } , Stdcall { unwind : bool } , Fastcall { unwind : bool } , Vectorcall { unwind : bool } , Thiscall { unwind : bool } , Aapcs { unwind : bool } , Win64 { unwind : bool } , SysV64 { unwind : bool } , PtxKernel , Msp430Interrupt , X86Interrupt , GpuKernel , EfiApi , AvrInterrupt , AvrNonBlockingInterrupt , CCmseNonSecureCall , CCmseNonSecureEntry , System { unwind : bool } , RustCall , Unadjusted , RustCold , RiscvInterruptM , RiscvInterruptS , RustInvalid , Custom , }
    };
}

Abi!();