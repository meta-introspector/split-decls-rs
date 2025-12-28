macro_rules! deps {
    () => {
        Running!();
        Cycle!();
    };
}

macro_rules! WaitForResult {
    () => {
        deps!();
        # [derive (Debug)] pub enum WaitForResult < 'me > { Running (Running < 'me >) , Available , Cycle { inner : bool } , }
    };
}

WaitForResult!()