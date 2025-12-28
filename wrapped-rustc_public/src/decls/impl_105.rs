macro_rules! deps {
    () => {
        CallConvention!();
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for CanonAbi { type T = CallConvention ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { match self { CanonAbi :: C => CallConvention :: C , CanonAbi :: Rust => CallConvention :: Rust , CanonAbi :: RustCold => CallConvention :: Cold , CanonAbi :: Custom => CallConvention :: Custom , CanonAbi :: Arm (arm_call) => match arm_call { ArmCall :: Aapcs => CallConvention :: ArmAapcs , ArmCall :: CCmseNonSecureCall => CallConvention :: CCmseNonSecureCall , ArmCall :: CCmseNonSecureEntry => CallConvention :: CCmseNonSecureEntry , } , CanonAbi :: GpuKernel => CallConvention :: GpuKernel , CanonAbi :: Interrupt (interrupt_kind) => match interrupt_kind { InterruptKind :: Avr => CallConvention :: AvrInterrupt , InterruptKind :: AvrNonBlocking => CallConvention :: AvrNonBlockingInterrupt , InterruptKind :: Msp430 => CallConvention :: Msp430Intr , InterruptKind :: RiscvMachine | InterruptKind :: RiscvSupervisor => { CallConvention :: RiscvInterrupt } InterruptKind :: X86 => CallConvention :: X86Intr , } , CanonAbi :: X86 (x86_call) => match x86_call { X86Call :: Fastcall => CallConvention :: X86Fastcall , X86Call :: Stdcall => CallConvention :: X86Stdcall , X86Call :: SysV64 => CallConvention :: X86_64SysV , X86Call :: Thiscall => CallConvention :: X86ThisCall , X86Call :: Vectorcall => CallConvention :: X86VectorCall , X86Call :: Win64 => CallConvention :: X86_64Win64 , } , } } }
    };
}

impl_105!()