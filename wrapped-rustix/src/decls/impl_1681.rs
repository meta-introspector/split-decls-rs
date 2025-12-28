macro_rules! deps {
    () => {
        Result!();
        Timespec!();
    };
}

macro_rules! impl_1681 {
    () => {
        deps!();
        impl TryFrom < Timespec > for Duration { type Error = TryFromIntError ; fn try_from (ts : Timespec) -> Result < Self , Self :: Error > { Ok (Self :: new (ts . tv_sec . try_into () ? , ts . tv_nsec as _)) } }
    };
}

impl_1681!()