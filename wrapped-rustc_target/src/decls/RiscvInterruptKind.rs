macro_rules! RiscvInterruptKind {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , Hash , Debug , HashStable_Generic)] pub enum RiscvInterruptKind { Machine , Supervisor , }
    };
}

RiscvInterruptKind!();