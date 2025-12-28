macro_rules! deps {
    () => {
        Timespec!();
        Result!();
    };
}

macro_rules! impl_1682 {
    () => {
        deps!();
        impl TryFrom < Duration > for Timespec { type Error = TryFromIntError ; fn try_from (dur : Duration) -> Result < Self , Self :: Error > { Ok (Self { tv_sec : dur . as_secs () . try_into () ? , tv_nsec : dur . subsec_nanos () as _ , }) } }
    };
}

impl_1682!()