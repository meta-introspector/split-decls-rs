// Generated macro for WithPart (struct)
macro_rules! Depcrate_parts_write_adapterWithPart {
() => {
// Module: crate::parts_write_adapter
// Provides: {"WithPart"}
// Dependencies: {}
# [doc = " A [`Writeable`] that writes out the given part."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use writeable::adapters::WithPart;"] # [doc = " use writeable::assert_writeable_parts_eq;"] # [doc = " use writeable::Part;"] # [doc = ""] # [doc = " // Simple usage:"] # [doc = ""] # [doc = " const PART: Part = Part {"] # [doc = "     category: \"foo\","] # [doc = "     value: \"bar\","] # [doc = " };"] # [doc = ""] # [doc = " assert_writeable_parts_eq!("] # [doc = "     WithPart {"] # [doc = "         writeable: \"Hello World\","] # [doc = "         part: PART"] # [doc = "     },"] # [doc = "     \"Hello World\","] # [doc = "     [(0, 11, PART)],"] # [doc = " );"] # [doc = ""] # [doc = " // Can be nested:"] # [doc = ""] # [doc = " const PART2: Part = Part {"] # [doc = "     category: \"foo2\","] # [doc = "     value: \"bar2\","] # [doc = " };"] # [doc = ""] # [doc = " assert_writeable_parts_eq!("] # [doc = "     WithPart {"] # [doc = "         writeable: WithPart {"] # [doc = "             writeable: \"Hello World\","] # [doc = "             part: PART"] # [doc = "         },"] # [doc = "         part: PART2"] # [doc = "     },"] # [doc = "     \"Hello World\","] # [doc = "     [(0, 11, PART), (0, 11, PART2)],"] # [doc = " );"] # [doc = " ```"] # [derive (Debug)] # [allow (clippy :: exhaustive_structs)] pub struct WithPart < T : ? Sized > { pub part : Part , pub writeable : T , }
};
}
