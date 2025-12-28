macro_rules! deps {
    () => {
        DeInteger!();
        Table!();
        Array!();
        DeArray!();
        DeString!();
        DeFloat!();
        DeTable!();
    };
}

macro_rules! DeValue {
    () => {
        deps!();
        # [doc = " Representation of a TOML value."] # [derive (Clone , Debug)] pub enum DeValue < 'i > { # [doc = " Represents a TOML string"] String (DeString < 'i >) , # [doc = " Represents a TOML integer"] Integer (DeInteger < 'i >) , # [doc = " Represents a TOML float"] Float (DeFloat < 'i >) , # [doc = " Represents a TOML boolean"] Boolean (bool) , # [doc = " Represents a TOML datetime"] Datetime (Datetime) , # [doc = " Represents a TOML array"] Array (DeArray < 'i >) , # [doc = " Represents a TOML table"] Table (DeTable < 'i >) , }
    };
}

DeValue!()