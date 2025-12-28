macro_rules! deps {
    () => {
        RefCount!();
        Config!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < C : cfg :: Config > RefCount < C > { pub (crate) const MAX : usize = Self :: BITS - 1 ; # [inline] fn incr (self) -> Option < Self > { if self . value >= Self :: MAX { test_println ! ("-> get: {}; MAX={}" , self . value , RefCount ::< C >:: MAX) ; return None ; } Some (Self :: from_usize (self . value + 1)) } # [inline] fn decr (self) -> Self { Self :: from_usize (self . value - 1) } }
    };
}

impl_104!()