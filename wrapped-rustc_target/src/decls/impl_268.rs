macro_rules! deps {
    () => {
        ABI!();
        PassMode!();
    };
}

macro_rules! impl_268 {
    () => {
        deps!();
        impl PassMode { # [doc = " Checks if these two `PassMode` are equal enough to be considered \"the same for all"] # [doc = " function call ABIs\". However, the `Layout` can also impact ABI decisions,"] # [doc = " so that needs to be compared as well!"] pub fn eq_abi (& self , other : & Self) -> bool { match (self , other) { (PassMode :: Ignore , PassMode :: Ignore) => true , (PassMode :: Direct (a1) , PassMode :: Direct (a2)) => a1 . eq_abi (a2) , (PassMode :: Pair (a1 , b1) , PassMode :: Pair (a2 , b2)) => a1 . eq_abi (a2) && b1 . eq_abi (b2) , (PassMode :: Cast { cast : c1 , pad_i32 : pad1 } , PassMode :: Cast { cast : c2 , pad_i32 : pad2 } ,) => c1 . eq_abi (c2) && pad1 == pad2 , (PassMode :: Indirect { attrs : a1 , meta_attrs : None , on_stack : s1 } , PassMode :: Indirect { attrs : a2 , meta_attrs : None , on_stack : s2 } ,) => a1 . eq_abi (a2) && s1 == s2 , (PassMode :: Indirect { attrs : a1 , meta_attrs : Some (e1) , on_stack : s1 } , PassMode :: Indirect { attrs : a2 , meta_attrs : Some (e2) , on_stack : s2 } ,) => a1 . eq_abi (a2) && e1 . eq_abi (e2) && s1 == s2 , _ => false , } } }
    };
}

impl_268!()