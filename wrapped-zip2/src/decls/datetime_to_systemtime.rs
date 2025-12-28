macro_rules! deps {
    () => {
        DateTime!();
    };
}

macro_rules! datetime_to_systemtime {
    () => {
        deps!();
        # [cfg (feature = "chrono")] # [doc = " Generate a `SystemTime` from a `DateTime`."] fn datetime_to_systemtime (datetime : & DateTime) -> Option < std :: time :: SystemTime > { if let Some (t) = generate_chrono_datetime (datetime) { let time = chrono :: DateTime :: < chrono :: Utc > :: from_naive_utc_and_offset (t , chrono :: Utc) ; return Some (time . into ()) ; } None }
    };
}

datetime_to_systemtime!();