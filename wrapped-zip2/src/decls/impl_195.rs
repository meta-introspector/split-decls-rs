macro_rules! deps {
    () => {
        DateTime!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        # [cfg (feature = "jiff-02")] impl TryFrom < DateTime > for civil :: DateTime { type Error = jiff :: Error ; fn try_from (value : DateTime) -> Result < Self , Self :: Error > { Self :: new (value . year () as i16 , value . month () as i8 , value . day () as i8 , value . hour () as i8 , value . minute () as i8 , value . second () as i8 , 0 ,) } }
    };
}

impl_195!();