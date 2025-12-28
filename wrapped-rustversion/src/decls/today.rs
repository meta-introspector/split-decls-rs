macro_rules! deps {
    () => {
        Date!();
    };
}

macro_rules! today {
    () => {
        deps!();
        pub fn today () -> Date { let default = Date { year : 2025 , month : 2 , day : 25 , } ; try_today () . unwrap_or (default) }
    };
}

today!()