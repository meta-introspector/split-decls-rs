use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Type alias for the result of `fallback_fluent_bundle` - a reference-counted pointer to a lazily"] # [doc = " evaluated fluent bundle."] pub type LazyFallbackBundle = Arc < LazyLock < FluentBundle , Box < dyn FnOnce () -> FluentBundle + DynSend > > > ;