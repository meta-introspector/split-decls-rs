macro_rules! deps {
    () => {
        Job!();
        JobRefId!();
        ArcJob!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < BODY > Job for ArcJob < BODY > where BODY : Fn (JobRefId) + Send + Sync , { unsafe fn execute (this : * const ()) { let pointer = this . expose_provenance () ; let this = unsafe { Arc :: from_raw (this as * mut Self) } ; (this . job) (JobRefId { pointer }) ; } }
    };
}

impl_45!()