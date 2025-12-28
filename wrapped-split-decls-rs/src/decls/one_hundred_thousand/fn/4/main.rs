use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn main () -> Result < () , Box < dyn std :: error :: Error > > { let agent = PyroscopeAgent :: builder ("http://localhost:4040" , "myapp-profile") . backend (pprof_backend (PprofConfig :: new () . sample_rate (100))) . build () ? ; let agent_running = agent . start () ? ; let n = 100_000 ; let mut page_rank = Pagerank :: new (n) ; let mut rng = StdRng :: seed_from_u64 (5) ; for from in 0 .. n { for _ in 0 .. rng . gen_range (0 .. 400) { let mut to = rng . gen_range (0 .. n) ; if to > 80_000 { to = rng . gen_range (0 .. 3) ; } page_rank . link (from , to) ? ; } } let _result = page_rank . rank (0.85 , 0.001) ; let agent_ready = agent_running . stop () ? ; agent_ready . shutdown () ; Ok (()) }
}