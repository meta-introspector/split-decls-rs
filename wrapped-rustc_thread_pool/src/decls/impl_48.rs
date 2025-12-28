macro_rules! deps {
    () => {
        JobFifo!();
        JobRef!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl JobFifo { pub (super) fn new () -> Self { JobFifo { inner : Injector :: new () } } pub (super) unsafe fn push (& self , job_ref : JobRef) -> JobRef { self . inner . push (job_ref) ; unsafe { JobRef :: new (self) } } }
    };
}

impl_48!();