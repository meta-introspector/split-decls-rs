macro_rules! deps {
    () => {
        Timestamp!();
    };
}

macro_rules! test_conversion {
    () => {
        deps!();
        # [doc = " Tests for conversion between `std::time::SystemTime` and `Timestamp`."] # [cfg (all (test , feature = "std" , not (miri)))] mod test_conversion { use std :: time :: { Duration , SystemTime } ; use super :: Timestamp ; const KNOWN_SECONDS : u64 = 1_501_520_400 ; const KNOWN_NANOS : u32 = 1_000 ; fn known_system_time () -> SystemTime { SystemTime :: UNIX_EPOCH . checked_add (Duration :: new (KNOWN_SECONDS , KNOWN_NANOS)) . unwrap () } fn known_timestamp () -> Timestamp { Timestamp :: from_unix_time (KNOWN_SECONDS , KNOWN_NANOS , 0 , 0) } # [test] fn to_system_time () { let st : SystemTime = known_timestamp () . into () ; assert_eq ! (known_system_time () , st) ; } # [test] fn from_system_time () { let ts : Timestamp = known_system_time () . try_into () . unwrap () ; assert_eq ! (known_timestamp () , ts) ; } # [test] fn from_system_time_before_epoch () { let before_epoch = match SystemTime :: UNIX_EPOCH . checked_sub (Duration :: from_nanos (1_000)) { Some (st) => st , None => return , } ; Timestamp :: try_from (before_epoch) . expect_err ("Timestamp should not be created from before epoch") ; } }
    };
}

test_conversion!();