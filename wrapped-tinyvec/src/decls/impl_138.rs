macro_rules! deps {
    () => {
        Array!();
        TinyVecSplice!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl < 'p , A , I > Iterator for TinyVecSplice < 'p , A , I > where A : Array , I : Iterator < Item = A :: Item > , { type Item = A :: Item ; # [inline] fn next (& mut self) -> Option < A :: Item > { if self . removal_start < self . removal_end { match self . replacement . next () { Some (replacement) => { let removed = core :: mem :: replace (& mut self . parent [self . removal_start] , replacement ,) ; self . removal_start += 1 ; Some (removed) } None => { let removed = self . parent . remove (self . removal_start) ; self . removal_end -= 1 ; Some (removed) } } } else { None } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let len = self . len () ; (len , Some (len)) } }
    };
}

impl_138!();