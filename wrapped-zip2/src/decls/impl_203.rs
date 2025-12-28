macro_rules! deps {
    () => {
        DateTime!();
    };
}

macro_rules! impl_203 {
    () => {
        deps!();
        # [cfg (feature = "time")] impl TryFrom < DateTime > for OffsetDateTime { type Error = ComponentRange ; fn try_from (dt : DateTime) -> Result < Self , Self :: Error > { PrimitiveDateTime :: try_from (dt) . map (PrimitiveDateTime :: assume_utc) } }
    };
}

impl_203!();