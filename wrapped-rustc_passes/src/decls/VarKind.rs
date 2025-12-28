macro_rules! deps {
    () => {
        LocalInfo!();
    };
}

macro_rules! VarKind {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug)] enum VarKind { Param (HirId , Symbol) , Local (LocalInfo) , Upvar (HirId , Symbol) , }
    };
}

VarKind!();