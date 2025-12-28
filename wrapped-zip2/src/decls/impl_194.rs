macro_rules! deps {
    () => {
        DateTime!();
        DateTimeRangeError!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        # [cfg (feature = "jiff-02")] impl TryFrom < civil :: DateTime > for DateTime { type Error = DateTimeRangeError ; fn try_from (value : civil :: DateTime) -> Result < Self , Self :: Error > { Self :: from_date_and_time (value . year () . try_into () ? , value . month () as u8 , value . day () as u8 , value . hour () as u8 , value . minute () as u8 , value . second () as u8 ,) } }
    };
}

impl_194!()