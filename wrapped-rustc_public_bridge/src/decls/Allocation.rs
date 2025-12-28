macro_rules! deps {
    () => {
        CompilerCtxt!();
        Bridge!();
        Tables!();
    };
}

macro_rules! Allocation {
    () => {
        deps!();
        pub trait Allocation < B : Bridge > { fn new < 'tcx > (bytes : Vec < Option < u8 > > , ptrs : Vec < (usize , rustc_middle :: mir :: interpret :: AllocId) > , align : u64 , mutability : rustc_middle :: mir :: Mutability , tables : & mut Tables < 'tcx , B > , cx : & CompilerCtxt < 'tcx , B > ,) -> Self ; }
    };
}

Allocation!();