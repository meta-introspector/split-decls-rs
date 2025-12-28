macro_rules! deps {
    () => {
        SsaVisitor!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl SsaVisitor < '_ , '_ > { fn check_dominates (& mut self , local : Local , loc : Location) { let set = & mut self . assignments [local] ; let assign_dominates = match * set { Set1 :: Empty | Set1 :: Many => false , Set1 :: One (def) => def . dominates (loc , self . dominators) , } ; if ! assign_dominates { * set = Set1 :: Many ; } } }
    };
}

impl_126!();