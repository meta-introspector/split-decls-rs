macro_rules! deps {
    () => {
        DateTime!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl DateTime { # [doc = " Returns the current time if possible, otherwise the default of 1980-01-01."] # [cfg (feature = "time")] pub fn default_for_write () -> Self { let now = OffsetDateTime :: now_utc () ; PrimitiveDateTime :: new (now . date () , now . time ()) . try_into () . unwrap_or_else (| _ | DateTime :: default ()) } # [doc = " Returns the current time if possible, otherwise the default of 1980-01-01."] # [cfg (not (feature = "time"))] pub fn default_for_write () -> Self { DateTime :: default () } }
    };
}

impl_190!()