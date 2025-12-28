macro_rules! SsaVisitor {
    () => {
        struct SsaVisitor < 'a , 'tcx > { body : & 'a Body < 'tcx > , dominators : & 'a Dominators < BasicBlock > , assignments : IndexVec < Local , Set1 < DefLocation > > , assignment_order : Vec < Local > , direct_uses : IndexVec < Local , u32 > , borrowed_locals : DenseBitSet < Local > , }
    };
}

SsaVisitor!();