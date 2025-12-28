macro_rules! deps {
    () => {
        DateTime!();
    };
}

macro_rules! impl_204 {
    () => {
        deps!();
        # [cfg (feature = "time")] impl TryFrom < DateTime > for PrimitiveDateTime { type Error = ComponentRange ; fn try_from (dt : DateTime) -> Result < Self , Self :: Error > { let date = Date :: from_calendar_date (dt . year () as i32 , Month :: try_from (dt . month ()) ? , dt . day ()) ? ; let time = Time :: from_hms (dt . hour () , dt . minute () , dt . second ()) ? ; Ok (PrimitiveDateTime :: new (date , time)) } }
    };
}

impl_204!();