macro_rules! deps {
    () => {
        Table!();
        Value!();
    };
}

macro_rules! SerializationStrategy {
    () => {
        deps!();
        # [derive (Copy , Clone , PartialEq , Eq , Debug)] pub (crate) enum SerializationStrategy { Value , Table , ArrayOfTables , Skip , Unknown , }
    };
}

SerializationStrategy!();