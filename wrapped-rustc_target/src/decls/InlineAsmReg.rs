macro_rules! InlineAsmReg {
    () => {
        # [derive (Copy , Clone , Debug , Eq , PartialEq , PartialOrd , Hash)] # [derive (HashStable_Generic , Encodable , Decodable)] pub enum InlineAsmReg { X86 (X86InlineAsmReg) , Arm (ArmInlineAsmReg) , AArch64 (AArch64InlineAsmReg) , RiscV (RiscVInlineAsmReg) , Nvptx (NvptxInlineAsmReg) , PowerPC (PowerPCInlineAsmReg) , Hexagon (HexagonInlineAsmReg) , LoongArch (LoongArchInlineAsmReg) , Mips (MipsInlineAsmReg) , S390x (S390xInlineAsmReg) , Sparc (SparcInlineAsmReg) , SpirV (SpirVInlineAsmReg) , Wasm (WasmInlineAsmReg) , Bpf (BpfInlineAsmReg) , Avr (AvrInlineAsmReg) , Msp430 (Msp430InlineAsmReg) , M68k (M68kInlineAsmReg) , CSKY (CSKYInlineAsmReg) , Err , }
    };
}

InlineAsmReg!();