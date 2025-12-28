macro_rules! deps {
    () => {
        DateTimeRangeError!();
        DateTime!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        # [cfg (feature = "chrono")] impl TryFrom < NaiveDateTime > for DateTime { type Error = DateTimeRangeError ; fn try_from (value : NaiveDateTime) -> Result < Self , Self :: Error > { DateTime :: from_date_and_time (value . year () . try_into () ? , value . month () . try_into () ? , value . day () . try_into () ? , value . hour () . try_into () ? , value . minute () . try_into () ? , value . second () . try_into () ? ,) } }
    };
}

impl_192!();