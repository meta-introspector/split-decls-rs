use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A reference to the SelfProfiler. It can be cloned and sent across thread"] # [doc = " boundaries at will."] # [derive (Clone)] pub struct SelfProfilerRef { profiler : Option < Arc < SelfProfiler > > , event_filter_mask : EventFilter , print_verbose_generic_activities : Option < TimePassesFormat > , }