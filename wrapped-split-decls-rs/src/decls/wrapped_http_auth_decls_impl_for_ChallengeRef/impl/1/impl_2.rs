use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'i > ChallengeRef < 'i > { pub fn new (scheme : & 'i str) -> Self { ChallengeRef { scheme , params : Vec :: new () , } } }