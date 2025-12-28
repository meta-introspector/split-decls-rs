macro_rules! deps {
    () => {
        DateTime!();
        DateTimeRangeError!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        # [cfg (feature = "time")] impl TryFrom < OffsetDateTime > for DateTime { type Error = DateTimeRangeError ; fn try_from (dt : OffsetDateTime) -> Result < Self , Self :: Error > { Self :: try_from (PrimitiveDateTime :: new (dt . date () , dt . time ())) } }
    };
}

impl_201!();