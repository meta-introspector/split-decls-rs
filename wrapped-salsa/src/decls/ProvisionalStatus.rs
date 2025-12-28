macro_rules! deps {
    () => {
        IterationCount!();
        CycleHeads!();
        Revision!();
    };
}

macro_rules! ProvisionalStatus {
    () => {
        deps!();
        # [derive (Debug)] pub enum ProvisionalStatus < 'db > { Provisional { iteration : IterationCount , verified_at : Revision , cycle_heads : & 'db CycleHeads , } , Final { iteration : IterationCount , verified_at : Revision , } , FallbackImmediate , }
    };
}

ProvisionalStatus!()