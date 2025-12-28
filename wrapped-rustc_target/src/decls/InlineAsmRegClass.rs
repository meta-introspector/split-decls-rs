macro_rules! InlineAsmRegClass {
    () => {
        # [derive (Copy , Clone , Debug , Eq , PartialEq , PartialOrd , Hash)] # [derive (HashStable_Generic , Encodable , Decodable)] pub enum InlineAsmRegClass { X86 (X86InlineAsmRegClass) , Arm (ArmInlineAsmRegClass) , AArch64 (AArch64InlineAsmRegClass) , RiscV (RiscVInlineAsmRegClass) , Nvptx (NvptxInlineAsmRegClass) , PowerPC (PowerPCInlineAsmRegClass) , Hexagon (HexagonInlineAsmRegClass) , LoongArch (LoongArchInlineAsmRegClass) , Mips (MipsInlineAsmRegClass) , S390x (S390xInlineAsmRegClass) , Sparc (SparcInlineAsmRegClass) , SpirV (SpirVInlineAsmRegClass) , Wasm (WasmInlineAsmRegClass) , Bpf (BpfInlineAsmRegClass) , Avr (AvrInlineAsmRegClass) , Msp430 (Msp430InlineAsmRegClass) , M68k (M68kInlineAsmRegClass) , CSKY (CSKYInlineAsmRegClass) , Err , }
    };
}

InlineAsmRegClass!()