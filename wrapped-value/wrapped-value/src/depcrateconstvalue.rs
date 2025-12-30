// Generated macro for ConstValue (enum)
macro_rules! DepcrateConstValue {
() => {
// Module: crate
// Provides: {"ConstValue"}
// Dependencies: {}
# [doc = " A resolved GraphQL value, for example `1` or `\"Hello World!\"`."] # [doc = ""] # [doc = " It can be serialized and deserialized. Enums will be converted to strings."] # [doc = " Attempting to serialize `Upload` will fail, and `Enum` and `Upload` cannot"] # [doc = " be deserialized."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/June2018/#Value)."] # [derive (Clone , Debug , Eq)] pub enum ConstValue { # [doc = " `null`."] Null , # [doc = " A number."] Number (Number) , # [doc = " A string."] String (String) , # [doc = " A boolean."] Boolean (bool) , # [doc = " A binary."] Binary (Bytes) , # [doc = " An enum. These are typically in `SCREAMING_SNAKE_CASE`."] Enum (Name) , # [doc = " A list of values."] List (Vec < ConstValue >) , # [doc = " An object. This is a map of keys to values."] Object (IndexMap < Name , ConstValue >) , }
};
}
