macro_rules! deps {
    () => {
        Replacements!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < I : Iterator < Item = char > > Iterator for Replacements < I > { type Item = char ; # [inline] fn next (& mut self) -> Option < char > { if let Some (c) = self . buffer . take () { return Some (c) ; } match self . iter . next () { Some (ch) => { let mut buffer = ArrayVec :: < [char ; 2] > :: new () ; super :: char :: decompose_cjk_compat_variants (ch , | d | buffer . push (d)) ; self . buffer = buffer . get (1) . copied () ; Some (buffer [0]) } None => None , } } fn size_hint (& self) -> (usize , Option < usize >) { let (lower , _) = self . iter . size_hint () ; (lower , None) } }
    };
}

impl_78!()