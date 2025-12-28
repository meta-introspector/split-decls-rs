macro_rules! deps {
    () => {
        TinyVec!();
        Array!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl < A : Array > Extend < A :: Item > for TinyVec < A > { # [inline] fn extend < T : IntoIterator < Item = A :: Item > > (& mut self , iter : T) { let iter = iter . into_iter () ; let (lower_bound , _) = iter . size_hint () ; self . reserve (lower_bound) ; let a = match self { TinyVec :: Heap (h) => return h . extend (iter) , TinyVec :: Inline (a) => a , } ; let mut iter = a . fill (iter) ; let maybe = iter . next () ; let surely = match maybe { Some (x) => x , None => return , } ; let mut v = a . drain_to_vec_and_reserve (a . len ()) ; v . push (surely) ; v . extend (iter) ; * self = TinyVec :: Heap (v) ; } }
    };
}

impl_147!()