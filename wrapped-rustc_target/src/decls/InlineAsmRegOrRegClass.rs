macro_rules! deps {
    () => {
        InlineAsmRegClass!();
        InlineAsmReg!();
    };
}

macro_rules! InlineAsmRegOrRegClass {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug , Eq , PartialEq , PartialOrd , Hash)] # [derive (HashStable_Generic , Encodable , Decodable)] pub enum InlineAsmRegOrRegClass { Reg (InlineAsmReg) , RegClass (InlineAsmRegClass) , }
    };
}

InlineAsmRegOrRegClass!()