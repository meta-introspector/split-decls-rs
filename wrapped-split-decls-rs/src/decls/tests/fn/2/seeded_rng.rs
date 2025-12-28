use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn seeded_rng () -> XorShiftRng { let mut seed = < XorShiftRng as SeedableRng > :: Seed :: default () ; (0 ..) . zip (seed . as_mut ()) . for_each (| (i , x) | * x = i) ; XorShiftRng :: from_seed (seed) }
}