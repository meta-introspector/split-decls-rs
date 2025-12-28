macro_rules! deps {
    () => {
        LockData!();
    };
}

macro_rules! Locks {
    () => {
        deps!();
        # [derive (Clone)] pub (crate) struct Locks { arc : Arc < LockData > , # [cfg (feature = "logging")] pub (crate) name : String , }
    };
}

Locks!();