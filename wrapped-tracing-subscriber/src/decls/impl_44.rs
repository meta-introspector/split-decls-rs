macro_rules! deps {
    () => {
        RecordFieldsMarker!();
        RecordFields!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < F > crate :: sealed :: Sealed < RecordFieldsMarker > for & F where F : RecordFields { }
    };
}

impl_44!()