// Generated macro for Part (struct)
macro_rules! DepcratePart {
() => {
// Module: crate
// Provides: {"Part"}
// Dependencies: {}
# [doc = " [`Part`]s are used as annotations for formatted strings."] # [doc = ""] # [doc = " For example, a string like `Alice, Bob` could assign a `NAME` part to the"] # [doc = " substrings `Alice` and `Bob`, and a `PUNCTUATION` part to `, `. This allows"] # [doc = " for example to apply styling only to names."] # [doc = ""] # [doc = " `Part` contains two fields, whose usage is left up to the producer of the [`Writeable`]."] # [doc = " Conventionally, the `category` field will identify the formatting logic that produces"] # [doc = " the string/parts, whereas the `value` field will have semantic meaning. `NAME` and"] # [doc = " `PUNCTUATION` could thus be defined as"] # [doc = " ```"] # [doc = " # use writeable::Part;"] # [doc = " const NAME: Part = Part {"] # [doc = "     category: \"userlist\","] # [doc = "     value: \"name\","] # [doc = " };"] # [doc = " const PUNCTUATION: Part = Part {"] # [doc = "     category: \"userlist\","] # [doc = "     value: \"punctuation\","] # [doc = " };"] # [doc = " ```"] # [doc = ""] # [doc = " That said, consumers should not usually have to inspect `Part` internals. Instead,"] # [doc = " formatters should expose the `Part`s they produces as constants."] # [derive (Clone , Copy , Debug , PartialEq)] # [allow (clippy :: exhaustive_structs)] pub struct Part { pub category : & 'static str , pub value : & 'static str , }
};
}
