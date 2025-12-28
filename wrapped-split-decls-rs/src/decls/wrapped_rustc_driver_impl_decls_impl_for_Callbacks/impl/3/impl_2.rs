use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Callbacks for TimePassesCallbacks { # [allow (rustc :: bad_opt_access)] fn config (& mut self , config : & mut interface :: Config) { self . time_passes = (config . opts . prints . is_empty () && config . opts . unstable_opts . time_passes) . then_some (config . opts . unstable_opts . time_passes_format) ; config . opts . trimmed_def_paths = true ; } }