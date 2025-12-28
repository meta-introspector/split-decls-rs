macro_rules! deps {
    () => {
        RecordFieldsMarker!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl crate :: sealed :: Sealed < RecordFieldsMarker > for Record < '_ > { }
    };
}

impl_42!()