macro_rules! deps {
    () => {
        Pending!();
    };
}

macro_rules! PollResult {
    () => {
        deps!();
        # [doc = " Possible results of the [`poll_signal`][SignalIterator::poll_signal] function."] pub enum PollResult < O > { # [doc = " A signal arrived"] Signal (O) , # [doc = " There are no signals yet but there may arrive some in the future"] Pending , # [doc = " The iterator was closed. There won't be any signals reported from now on."] Closed , # [doc = " An error happened during polling for arrived signals."] Err (Error) , }
    };
}

PollResult!();