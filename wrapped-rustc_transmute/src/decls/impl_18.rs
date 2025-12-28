macro_rules! deps {
    () => {
        Byte!();
        UnionIter!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < S : Copy , X : Iterator < Item = (Byte , S) > , Y : Iterator < Item = (Byte , S) > > Iterator for UnionIter < X , Y > { type Item = (Byte , (Option < S > , Option < S >)) ; fn next (& mut self) -> Option < Self :: Item > { use std :: cmp :: { self , Ordering } ; let ret ; match (self . xs . peek_mut () , self . ys . peek_mut ()) { (None , None) => { ret = None ; } (Some (x) , None) => { ret = Some ((x . 0 , (Some (x . 1) , None))) ; self . xs . next () ; } (None , Some (y)) => { ret = Some ((y . 0 , (None , Some (y . 1)))) ; self . ys . next () ; } (Some (x) , Some (y)) => { let start ; let end ; let dst ; match x . 0 . start . cmp (& y . 0 . start) { Ordering :: Less => { start = x . 0 . start ; end = cmp :: min (x . 0 . end , y . 0 . start) ; dst = (Some (x . 1) , None) ; } Ordering :: Greater => { start = y . 0 . start ; end = cmp :: min (x . 0 . start , y . 0 . end) ; dst = (None , Some (y . 1)) ; } Ordering :: Equal => { start = x . 0 . start ; end = cmp :: min (x . 0 . end , y . 0 . end) ; dst = (Some (x . 1) , Some (y . 1)) ; } } ret = Some ((Byte { start , end } , dst)) ; if start == x . 0 . start { x . 0 . start = end ; } if start == y . 0 . start { y . 0 . start = end ; } if x . 0 . is_empty () { self . xs . next () ; } if y . 0 . is_empty () { self . ys . next () ; } } } ret } }
    };
}

impl_18!()