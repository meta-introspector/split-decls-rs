macro_rules! deps {
    () => {
        Uuid!();
    };
}

macro_rules! macro_3 {
    () => {
        deps!();
        define_uuid_macro ! { # [doc = " Parse [`Uuid`][uuid::Uuid]s from string literals at compile time."] # [doc = ""] # [doc = " ## Usage"] # [doc = ""] # [doc = " This macro transforms the string literal representation of a"] # [doc = " [`Uuid`][uuid::Uuid] into the bytes representation, raising a compilation"] # [doc = " error if it cannot properly be parsed."] # [doc = ""] # [doc = " ## Examples"] # [doc = ""] # [doc = " Setting a global constant:"] # [doc = ""] # [doc = " ```"] # [doc = " # use uuid::{uuid, Uuid};"] # [doc = " pub const SCHEMA_ATTR_CLASS: Uuid = uuid!(\"00000000-0000-0000-0000-ffff00000000\");"] # [doc = " pub const SCHEMA_ATTR_UUID: Uuid = uuid!(\"00000000-0000-0000-0000-ffff00000001\");"] # [doc = " pub const SCHEMA_ATTR_NAME: Uuid = uuid!(\"00000000-0000-0000-0000-ffff00000002\");"] # [doc = " ```"] # [doc = ""] # [doc = " Defining a local variable:"] # [doc = ""] # [doc = " ```"] # [doc = " # use uuid::uuid;"] # [doc = " let uuid = uuid!(\"urn:uuid:F9168C5E-CEB2-4faa-B6BF-329BF39FA1E4\");"] # [doc = " ```"] # [doc = " Using a const variable:"] # [doc = " ```"] # [doc = " # use uuid::uuid;"] # [doc = " const UUID_STR: &str = \"12345678-1234-5678-1234-567812345678\";"] # [doc = " let UUID = uuid!(UUID_STR);"] # [doc = " ```"] # [doc = ""] # [doc = " ## Compilation Failures"] # [doc = ""] # [doc = " Invalid UUIDs are rejected:"] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " # use uuid::uuid;"] # [doc = " let uuid = uuid!(\"F9168C5E-ZEB2-4FAA-B6BF-329BF39FA1E4\");"] # [doc = " ```"] # [doc = ""] # [doc = " Enable the feature `macro-diagnostics` to see the error messages below."] # [doc = ""] # [doc = " Provides the following compilation error:"] # [doc = ""] # [doc = " ```txt"] # [doc = " error: invalid character: expected an optional prefix of `urn:uuid:` followed by [0-9a-fA-F-], found Z at 9"] # [doc = "     |"] # [doc = "     |     let id = uuid!(\"F9168C5E-ZEB2-4FAA-B6BF-329BF39FA1E4\");"] # [doc = "     |                              ^"] # [doc = " ```"] # [doc = ""] # [doc = " [uuid::Uuid]: https://docs.rs/uuid/*/uuid/struct.Uuid.html"] }
    };
}

macro_3!();