use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn take_array < T , const N : usize > (b : & mut Box < [T] >) -> Result < [T ; N] , Box < [T] > > { let b : Box < [T ; N] > = std :: mem :: take (b) . try_into () ? ; Ok (* b) }