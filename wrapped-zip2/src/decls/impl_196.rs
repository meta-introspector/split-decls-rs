macro_rules! deps {
    () => {
        DateTimeRangeError!();
        DateTime!();
    };
}

macro_rules! impl_196 {
    () => {
        deps!();
        impl TryFrom < (u16 , u16) > for DateTime { type Error = DateTimeRangeError ; # [inline] fn try_from (values : (u16 , u16)) -> Result < Self , Self :: Error > { Self :: try_from_msdos (values . 0 , values . 1) } }
    };
}

impl_196!();