macro_rules! deps {
    () => {
        Decompositions!();
        DecompositionType!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < I : Iterator < Item = char > > Iterator for Decompositions < I > { type Item = char ; # [inline] fn next (& mut self) -> Option < char > { while self . ready . end == 0 { match (self . iter . next () , & self . kind) { (Some (ch) , & DecompositionType :: Canonical) => { super :: char :: decompose_canonical (ch , | d | self . push_back (d)) ; } (Some (ch) , & DecompositionType :: Compatible) => { super :: char :: decompose_compatible (ch , | d | self . push_back (d)) ; } (None , _) => { if self . buffer . is_empty () { return None ; } else { self . sort_pending () ; self . ready . end = self . buffer . len () ; break ; } } } } let (_ , ch) = self . buffer [self . ready . start] ; self . increment_next_ready () ; Some (ch) } fn size_hint (& self) -> (usize , Option < usize >) { let (lower , _) = self . iter . size_hint () ; (lower , None) } }
    };
}

impl_7!();