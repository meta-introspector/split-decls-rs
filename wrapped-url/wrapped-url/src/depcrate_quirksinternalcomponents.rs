// Generated macro for InternalComponents (struct)
macro_rules! Depcrate_quirksInternalComponents {
() => {
// Module: crate::quirks
// Provides: {"InternalComponents"}
// Dependencies: {}
# [doc = " Internal components / offsets of a URL."] # [doc = ""] # [doc = " https://user@pass:example.com:1234/foo/bar?baz#quux"] # [doc = "      |      |    |          | ^^^^|       |   |"] # [doc = "      |      |    |          | |   |       |   `----- fragment_start"] # [doc = "      |      |    |          | |   |       `--------- query_start"] # [doc = "      |      |    |          | |   `----------------- path_start"] # [doc = "      |      |    |          | `--------------------- port"] # [doc = "      |      |    |          `----------------------- host_end"] # [doc = "      |      |    `---------------------------------- host_start"] # [doc = "      |      `--------------------------------------- username_end"] # [doc = "      `---------------------------------------------- scheme_end"] # [derive (Copy , Clone)] # [cfg (feature = "expose_internals")] pub struct InternalComponents { pub scheme_end : u32 , pub username_end : u32 , pub host_start : u32 , pub host_end : u32 , pub port : Option < u16 > , pub path_start : u32 , pub query_start : Option < u32 > , pub fragment_start : Option < u32 > , }
};
}
