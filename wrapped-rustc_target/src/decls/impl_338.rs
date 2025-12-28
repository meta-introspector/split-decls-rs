macro_rules! deps {
    () => {
        Arch!();
        StackProbeType!();
        TargetEnv!();
    };
}

macro_rules! impl_338 {
    () => {
        deps!();
        impl Arch { fn target_name (self) -> & 'static str { match self { Armv7k => "armv7k" , Armv7s => "armv7s" , Arm64 => "arm64" , Arm64e => "arm64e" , Arm64_32 => "arm64_32" , I386 => "i386" , I686 => "i686" , X86_64 => "x86_64" , X86_64h => "x86_64h" , } } pub (crate) fn target_arch (self) -> Cow < 'static , str > { Cow :: Borrowed (match self { Armv7k | Armv7s => "arm" , Arm64 | Arm64e | Arm64_32 => "aarch64" , I386 | I686 => "x86" , X86_64 | X86_64h => "x86_64" , }) } fn target_cpu (self , env : TargetEnv) -> & 'static str { match self { Armv7k => "cortex-a8" , Armv7s => "swift" , Arm64 => match env { TargetEnv :: Normal => "apple-a7" , TargetEnv :: Simulator => "apple-a12" , TargetEnv :: MacCatalyst => "apple-a12" , } , Arm64e => "apple-a12" , Arm64_32 => "apple-s4" , I386 | I686 => "penryn" , X86_64 => "penryn" , X86_64h => "core-avx2" , } } fn stack_probes (self) -> StackProbeType { match self { Armv7k | Armv7s => StackProbeType :: None , Arm64 | Arm64e | Arm64_32 | I386 | I686 | X86_64 | X86_64h => StackProbeType :: Inline , } } }
    };
}

impl_338!();