macro_rules! deps {
    () => {
        DateTime!();
        DateTimeRangeError!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        # [cfg (feature = "chrono")] impl TryFrom < DateTime > for NaiveDateTime { type Error = DateTimeRangeError ; fn try_from (value : DateTime) -> Result < Self , Self :: Error > { let date = NaiveDate :: from_ymd_opt (value . year () . into () , value . month () . into () , value . day () . into () ,) . ok_or (DateTimeRangeError) ? ; let time = NaiveTime :: from_hms_opt (value . hour () . into () , value . minute () . into () , value . second () . into () ,) . ok_or (DateTimeRangeError) ? ; Ok (NaiveDateTime :: new (date , time)) } }
    };
}

impl_193!()