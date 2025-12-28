use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn default_shard_amount () -> usize { static DEFAULT_SHARD_AMOUNT : OnceLock < usize > = OnceLock :: new () ; * DEFAULT_SHARD_AMOUNT . get_or_init (| | { (std :: thread :: available_parallelism () . map_or (1 , usize :: from) * 4) . next_power_of_two () }) }