macro_rules! deps {
    () => {
        DateTime!();
        DateTimeRangeError!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        # [cfg (feature = "time")] impl TryFrom < PrimitiveDateTime > for DateTime { type Error = DateTimeRangeError ; fn try_from (dt : PrimitiveDateTime) -> Result < Self , Self :: Error > { Self :: from_date_and_time (dt . year () . try_into () ? , dt . month () . into () , dt . day () , dt . hour () , dt . minute () , dt . second () ,) } }
    };
}

impl_202!()