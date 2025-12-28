macro_rules! CallConvention {
    () => {
        # [doc = " General language calling conventions."] # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash , Serialize)] pub enum CallConvention { C , Rust , Cold , PreserveMost , PreserveAll , Custom , ArmAapcs , CCmseNonSecureCall , CCmseNonSecureEntry , Msp430Intr , PtxKernel , GpuKernel , X86Fastcall , X86Intr , X86Stdcall , X86ThisCall , X86VectorCall , X86_64SysV , X86_64Win64 , AvrInterrupt , AvrNonBlockingInterrupt , RiscvInterrupt , }
    };
}

CallConvention!();