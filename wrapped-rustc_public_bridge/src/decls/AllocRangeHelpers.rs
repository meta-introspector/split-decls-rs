macro_rules! AllocRangeHelpers {
    () => {
        pub trait AllocRangeHelpers < 'tcx > { fn alloc_range (& self , offset : rustc_abi :: Size , size : rustc_abi :: Size) -> AllocRange ; }
    };
}

AllocRangeHelpers!()