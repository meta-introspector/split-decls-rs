use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: load_workspace");
pub fn load_workspace (ws : ProjectWorkspace , extra_env : & FxHashMap < String , Option < String > > , load_config : & LoadCargoConfig ,) -> anyhow :: Result < (RootDatabase , vfs :: Vfs , Option < ProcMacroClient >) > { let lru_cap = std :: env :: var ("RA_LRU_CAP") . ok () . and_then (| it | it . parse :: < u16 > () . ok ()) ; let mut db = RootDatabase :: new (lru_cap) ; let (vfs , proc_macro_server) = load_workspace_into_db (ws , extra_env , load_config , & mut db) ? ; Ok ((db , vfs , proc_macro_server)) }
}