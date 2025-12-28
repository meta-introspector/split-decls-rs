macro_rules! deps {
    () => {
        Tree!();
        Answer!();
        Assume!();
        Reason!();
    };
}

macro_rules! size {
    () => {
        deps!();
        mod size { use super :: * ; # [test] fn size () { let small = Tree :: number (1) ; let large = Tree :: number (2) ; for alignment in [false , true] { for lifetimes in [false , true] { for safety in [false , true] { for validity in [false , true] { let assume = Assume { alignment , lifetimes , safety , validity } ; assert_eq ! (is_transmutable (& small , & large , assume) , Answer :: No (Reason :: DstIsTooBig) , "assume: {assume:?}") ; assert_eq ! (is_transmutable (& large , & small , assume) , Answer :: Yes , "assume: {assume:?}") ; } } } } } }
    };
}

size!()