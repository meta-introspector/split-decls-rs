macro_rules! deps {
    () => {
        OsVersion!();
    };
}

macro_rules! test {
    () => {
        deps!();
        # [cfg (test)] # [allow (clippy :: nonminimal_bool)] mod test { use super :: * ; use std :: sync :: RwLock ; static TEST_CURRENT : RwLock < OsVersion > = RwLock :: new (OsVersion :: new (0 , 0 , 0 , 0)) ; pub fn test_current () -> OsVersion { * TEST_CURRENT . read () . unwrap () } fn set_current (version : OsVersion) { * TEST_CURRENT . write () . unwrap () = version ; } # [test] fn test () { assert_eq ! (OsVersion :: current () , OsVersion :: new (0 , 0 , 0 , 0)) ; set_current (OsVersion :: new (1 , 2 , 3 , 4)) ; assert_eq ! (OsVersion :: current () , OsVersion :: new (1 , 2 , 3 , 4)) ; set_current (OsVersion :: new (10 , 0 , 0 , 0)) ; assert ! (OsVersion :: current () >= OsVersion :: new (9 , 0 , 0 , 0)) ; assert ! (OsVersion :: current () >= OsVersion :: new (10 , 0 , 0 , 0)) ; assert ! (! (OsVersion :: current () >= OsVersion :: new (11 , 0 , 0 , 0))) ; set_current (OsVersion :: new (10 , 100 , 0 , 0)) ; assert ! (OsVersion :: current () >= OsVersion :: new (10 , 99 , 0 , 0)) ; assert ! (OsVersion :: current () >= OsVersion :: new (10 , 100 , 0 , 0)) ; assert ! (! (OsVersion :: current () >= OsVersion :: new (10 , 101 , 0 , 0))) ; set_current (OsVersion :: new (10 , 100 , 1000 , 0)) ; assert ! (OsVersion :: current () >= OsVersion :: new (10 , 100 , 999 , 0)) ; assert ! (OsVersion :: current () >= OsVersion :: new (10 , 100 , 1000 , 0)) ; assert ! (! (OsVersion :: current () >= OsVersion :: new (10 , 100 , 1001 , 0))) ; set_current (OsVersion :: new (10 , 100 , 1_000 , 10_000)) ; assert ! (OsVersion :: current () >= OsVersion :: new (10 , 100 , 1_000 , 9_999)) ; assert ! (OsVersion :: current () >= OsVersion :: new (10 , 100 , 1_000 , 10_000)) ; assert ! (! (OsVersion :: current () >= OsVersion :: new (10 , 100 , 1_000 , 10_001))) ; } # [test] fn test_uncertain () { is_server () ; assert_ne ! (revision () , 0) ; } }
    };
}

test!()