macro_rules! deps {
    () => {
        Job!();
        JobFifo!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl Job for JobFifo { unsafe fn execute (this : * const ()) { let this = unsafe { & * (this as * const Self) } ; loop { match this . inner . steal () { Steal :: Success (job_ref) => break unsafe { job_ref . execute () } , Steal :: Empty => panic ! ("FIFO is empty") , Steal :: Retry => { } } } } }
    };
}

impl_49!();