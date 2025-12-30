// Generated macro for Config (struct)
macro_rules! Depcrate_searcherConfig {
() => {
// Module: crate::searcher
// Provides: {"Config"}
// Dependencies: {}
# [doc = " The internal configuration of a searcher. This is shared among several"] # [doc = " search related types, but is only ever written to by the SearcherBuilder."] # [derive (Clone , Debug)] pub struct Config { # [doc = " The line terminator to use."] line_term : LineTerminator , # [doc = " Whether to invert matching."] invert_match : bool , # [doc = " The number of lines after a match to include."] after_context : usize , # [doc = " The number of lines before a match to include."] before_context : usize , # [doc = " Whether to enable unbounded context or not."] passthru : bool , # [doc = " Whether to count line numbers."] line_number : bool , # [doc = " The maximum amount of heap memory to use."] # [doc = ""] # [doc = " When not given, no explicit limit is enforced. When set to `0`, then"] # [doc = " only the memory map search strategy is available."] heap_limit : Option < usize > , # [doc = " The memory map strategy."] mmap : MmapChoice , # [doc = " The binary data detection strategy."] binary : BinaryDetection , # [doc = " Whether to enable matching across multiple lines."] multi_line : bool , # [doc = " An encoding that, when present, causes the searcher to transcode all"] # [doc = " input from the encoding to UTF-8."] encoding : Option < Encoding > , # [doc = " Whether to do automatic transcoding based on a BOM or not."] bom_sniffing : bool , # [doc = " Whether to stop searching when a non-matching line is found after a"] # [doc = " matching line."] stop_on_nonmatch : bool , # [doc = " The maximum number of matches this searcher should emit."] max_matches : Option < u64 > , }
};
}
