macro_rules! InlineAsmClobberAbi {
    () => {
        # [derive (Copy , Clone , Debug , Eq , PartialEq , PartialOrd , Hash)] # [derive (HashStable_Generic , Encodable , Decodable)] pub enum InlineAsmClobberAbi { X86 , X86_64Win , X86_64SysV , Arm , AArch64 , AArch64NoX18 , Arm64EC , Avr , RiscV , RiscVE , LoongArch , PowerPC , S390x , Bpf , Msp430 , }
    };
}

InlineAsmClobberAbi!();