macro_rules! TransmuteMutDst {
    () => {
        pub trait TransmuteMutDst < 'a > { type Dst : ? Sized ; # [must_use] fn transmute_mut (self) -> & 'a mut Self :: Dst ; }
    };
}

TransmuteMutDst!()