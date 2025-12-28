macro_rules! deps {
    () => {
        CancellationToken!();
    };
}

macro_rules! macro_51 {
    () => {
        deps!();
        pin_project ! { # [doc = " A Future that is resolved once the corresponding [`CancellationToken`]"] # [doc = " is cancelled or a given Future gets resolved. It is biased towards the"] # [doc = " Future completion."] # [must_use = "futures do nothing unless polled"] pub (crate) struct RunUntilCancelledFutureOwned < F : Future > { # [pin] cancellation : WaitForCancellationFutureOwned , # [pin] future : F , } }
    };
}

macro_51!();