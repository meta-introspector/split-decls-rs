macro_rules! deps {
    () => {
        Exfiltrator!();
        Pending!();
        PollResult!();
        Forever!();
        SignalsInfo!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < 'a , E : Exfiltrator > Iterator for Forever < 'a , E > { type Item = E :: Output ; fn next (& mut self) -> Option < E :: Output > { loop { match self . 0 . poll_signal (& mut SignalsInfo :: < E > :: has_signals) { PollResult :: Signal (result) => break Some (result) , PollResult :: Closed => break None , PollResult :: Pending => continue , PollResult :: Err (error) => panic ! ("Unexpected error: {}" , error) , } } } }
    };
}

impl_49!();