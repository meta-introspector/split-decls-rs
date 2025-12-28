macro_rules! deps {
    () => {
        CancellationToken!();
    };
}

macro_rules! macro_48 {
    () => {
        deps!();
        pin_project ! { # [doc = " A Future that is resolved once the corresponding [`CancellationToken`]"] # [doc = " is cancelled or a given Future gets resolved. It is biased towards the"] # [doc = " Future completion."] # [must_use = "futures do nothing unless polled"] pub (crate) struct RunUntilCancelledFuture <'a , F : Future > { # [pin] cancellation : WaitForCancellationFuture <'a >, # [pin] future : F , } }
    };
}

macro_48!()