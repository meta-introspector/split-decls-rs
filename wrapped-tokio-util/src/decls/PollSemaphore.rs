macro_rules! deps {
    () => {
        ReusableBoxFuture!();
    };
}

macro_rules! PollSemaphore {
    () => {
        deps!();
        # [doc = " A wrapper around [`Semaphore`] that provides a `poll_acquire` method."] # [doc = ""] # [doc = " [`Semaphore`]: tokio::sync::Semaphore"] pub struct PollSemaphore { semaphore : Arc < Semaphore > , permit_fut : Option < (u32 , ReusableBoxFuture < 'static , Result < OwnedSemaphorePermit , AcquireError > > ,) > , }
    };
}

PollSemaphore!()