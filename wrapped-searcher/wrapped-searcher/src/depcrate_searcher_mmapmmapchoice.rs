// Generated macro for MmapChoice (struct)
macro_rules! Depcrate_searcher_mmapMmapChoice {
() => {
// Module: crate::searcher::mmap
// Provides: {"MmapChoice"}
// Dependencies: {}
# [doc = " Controls the strategy used for determining when to use memory maps."] # [doc = ""] # [doc = " If a searcher is called in circumstances where it is possible to use memory"] # [doc = " maps, and memory maps are enabled, then it will attempt to do so if it"] # [doc = " believes it will make the search faster."] # [doc = ""] # [doc = " By default, memory maps are disabled."] # [derive (Clone , Debug)] pub struct MmapChoice (MmapChoiceImpl) ;
};
}
