// Generated macro for SelfProfilerRef (struct)
macro_rules! Depcrate_profilingSelfProfilerRef {
() => {
// Module: crate::profiling
// Provides: {"SelfProfilerRef"}
// Dependencies: {}
# [doc = " A reference to the SelfProfiler. It can be cloned and sent across thread"] # [doc = " boundaries at will."] # [derive (Clone)] pub struct SelfProfilerRef { profiler : Option < Arc < SelfProfiler > > , event_filter_mask : EventFilter , print_verbose_generic_activities : Option < TimePassesFormat > , }
};
}
