macro_rules! deps {
    () => {
        RecordFieldsMarker!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl crate :: sealed :: Sealed < RecordFieldsMarker > for Event < '_ > { }
    };
}

impl_38!();