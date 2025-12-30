// Generated macro for Value (enum)
macro_rules! Depcrate_valueValue {
() => {
// Module: crate::value
// Provides: {"Value"}
// Dependencies: {}
# [doc = " Representation of a TOML value."] # [derive (PartialEq , Clone , Debug)] pub enum Value { # [doc = " Represents a TOML string"] String (String) , # [doc = " Represents a TOML integer"] Integer (i64) , # [doc = " Represents a TOML float"] Float (f64) , # [doc = " Represents a TOML boolean"] Boolean (bool) , # [doc = " Represents a TOML datetime"] Datetime (Datetime) , # [doc = " Represents a TOML array"] Array (Array) , # [doc = " Represents a TOML table"] Table (Table) , }
};
}
