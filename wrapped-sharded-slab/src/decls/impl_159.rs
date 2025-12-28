macro_rules! deps {
    () => {
        Config!();
        IterMut!();
        Shard!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl < 'a , T , C > Iterator for IterMut < 'a , T , C > where T : 'a , C : cfg :: Config + 'a , { type Item = & 'a Shard < T , C > ; fn next (& mut self) -> Option < Self :: Item > { test_println ! ("IterMut::next") ; loop { let next = self . 0 . next () ; test_println ! ("-> next.is_some={}" , next . is_some ()) ; if let Some (shard) = next ? . load (Acquire) { test_println ! ("-> done") ; return Some (shard) ; } } } }
    };
}

impl_159!();