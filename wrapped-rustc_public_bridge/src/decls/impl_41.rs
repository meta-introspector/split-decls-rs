macro_rules! deps {
    () => {
        Bridge!();
        AllocRangeHelpers!();
        CompilerCtxt!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < 'tcx , B : Bridge > AllocRangeHelpers < 'tcx > for CompilerCtxt < 'tcx , B > { fn alloc_range (& self , offset : rustc_abi :: Size , size : rustc_abi :: Size ,) -> mir :: interpret :: AllocRange { rustc_middle :: mir :: interpret :: alloc_range (offset , size) } }
    };
}

impl_41!()