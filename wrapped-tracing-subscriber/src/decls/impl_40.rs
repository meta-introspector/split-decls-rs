macro_rules! deps {
    () => {
        RecordFieldsMarker!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl crate :: sealed :: Sealed < RecordFieldsMarker > for Attributes < '_ > { }
    };
}

impl_40!();