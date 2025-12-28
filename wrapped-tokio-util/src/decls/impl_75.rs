macro_rules! deps {
    () => {
        PollSemaphore!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl AsRef < Semaphore > for PollSemaphore { fn as_ref (& self) -> & Semaphore { & self . semaphore } }
    };
}

impl_75!();