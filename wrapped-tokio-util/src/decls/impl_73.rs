macro_rules! deps {
    () => {
        PollSemaphore!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl Clone for PollSemaphore { fn clone (& self) -> PollSemaphore { PollSemaphore :: new (self . clone_inner ()) } }
    };
}

impl_73!();