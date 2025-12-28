macro_rules! deps {
    () => {
        CancellationToken!();
    };
}

macro_rules! macro_36 {
    () => {
        deps!();
        pin_project ! { # [doc = " A Future that is resolved once the corresponding [`CancellationToken`]"] # [doc = " is cancelled."] # [must_use = "futures do nothing unless polled"] pub struct WaitForCancellationFuture <'a > { cancellation_token : &'a CancellationToken , # [pin] future : tokio :: sync :: futures :: Notified <'a >, } }
    };
}

macro_36!()