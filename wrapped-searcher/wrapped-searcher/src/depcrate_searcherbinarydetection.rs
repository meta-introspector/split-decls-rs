// Generated macro for BinaryDetection (struct)
macro_rules! Depcrate_searcherBinaryDetection {
() => {
// Module: crate::searcher
// Provides: {"BinaryDetection"}
// Dependencies: {}
# [doc = " The behavior of binary detection while searching."] # [doc = ""] # [doc = " Binary detection is the process of _heuristically_ identifying whether a"] # [doc = " given chunk of data is binary or not, and then taking an action based on"] # [doc = " the result of that heuristic. The motivation behind detecting binary data"] # [doc = " is that binary data often indicates data that is undesirable to search"] # [doc = " using textual patterns. Of course, there are many cases in which this isn't"] # [doc = " true, which is why binary detection is disabled by default."] # [doc = ""] # [doc = " Unfortunately, binary detection works differently depending on the type of"] # [doc = " search being executed:"] # [doc = ""] # [doc = " 1. When performing a search using a fixed size buffer, binary detection is"] # [doc = "    applied to the buffer's contents as it is filled. Binary detection must"] # [doc = "    be applied to the buffer directly because binary files may not contain"] # [doc = "    line terminators, which could result in exorbitant memory usage."] # [doc = " 2. When performing a search using memory maps or by reading data off the"] # [doc = "    heap, then binary detection is only guaranteed to be applied to the"] # [doc = "    parts corresponding to a match. When `Quit` is enabled, then the first"] # [doc = "    few KB of the data are searched for binary data."] # [derive (Clone , Debug , Default , Eq , PartialEq)] pub struct BinaryDetection (line_buffer :: BinaryDetection) ;
};
}
