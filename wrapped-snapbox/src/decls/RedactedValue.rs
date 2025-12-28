macro_rules! deps {
    () => {
        RedactedValueInner!();
    };
}

macro_rules! RedactedValue {
    () => {
        deps!();
        # [derive (Clone)] pub struct RedactedValue { inner : Option < RedactedValueInner > , }
    };
}

RedactedValue!();