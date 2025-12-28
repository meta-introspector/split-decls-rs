macro_rules! TransmuteRefDst {
    () => {
        pub trait TransmuteRefDst < 'a > { type Dst : ? Sized ; # [must_use] fn transmute_ref (self) -> & 'a Self :: Dst ; }
    };
}

TransmuteRefDst!()