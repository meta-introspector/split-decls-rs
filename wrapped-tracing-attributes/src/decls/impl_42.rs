macro_rules! deps {
    () => {
        AsyncTraitBlockReplacer!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl VisitMut for AsyncTraitBlockReplacer < '_ > { fn visit_block_mut (& mut self , i : & mut Block) { if i == self . block { * i = self . patched_block . clone () ; } } }
    };
}

impl_42!()