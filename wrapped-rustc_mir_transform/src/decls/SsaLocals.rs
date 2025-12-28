macro_rules! SsaLocals {
    () => {
        pub (super) struct SsaLocals { # [doc = " Assignments to each local. This defines whether the local is SSA."] assignments : IndexVec < Local , Set1 < DefLocation > > , # [doc = " We visit the body in reverse postorder, to ensure each local is assigned before it is used."] # [doc = " We remember the order in which we saw the assignments to compute the SSA values in a single"] # [doc = " pass."] assignment_order : Vec < Local > , # [doc = " Copy equivalence classes between locals. See `copy_classes` for documentation."] copy_classes : IndexVec < Local , Local > , # [doc = " Number of \"direct\" uses of each local, ie. uses that are not dereferences."] # [doc = " We ignore non-uses (Storage statements, debuginfo)."] direct_uses : IndexVec < Local , u32 > , # [doc = " Set of SSA locals that are immutably borrowed."] borrowed_locals : DenseBitSet < Local > , }
    };
}

SsaLocals!()