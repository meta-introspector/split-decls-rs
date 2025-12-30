// Generated macro for Value (enum)
macro_rules! DepcrateValue {
() => {
// Module: crate
// Provides: {"Value"}
// Dependencies: {}
# [doc = " A GraphQL value, for example `1`, `$name` or `\"Hello World!\"`. This is"] # [doc = " [`ConstValue`](enum.ConstValue.html) with variables."] # [doc = ""] # [doc = " It can be serialized and deserialized. Enums will be converted to strings."] # [doc = " Attempting to serialize `Upload` or `Variable` will fail, and `Enum`,"] # [doc = " `Upload` and `Variable` cannot be deserialized."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/June2018/#Value)."] # [derive (Clone , Debug , PartialEq , Eq)] pub enum Value { # [doc = " A variable, without the `$`."] Variable (Name) , # [doc = " `null`."] Null , # [doc = " A number."] Number (Number) , # [doc = " A string."] String (String) , # [doc = " A boolean."] Boolean (bool) , # [doc = " A binary."] Binary (Bytes) , # [doc = " An enum. These are typically in `SCREAMING_SNAKE_CASE`."] Enum (Name) , # [doc = " A list of values."] List (Vec < Value >) , # [doc = " An object. This is a map of keys to values."] Object (IndexMap < Name , Value >) , }
};
}
