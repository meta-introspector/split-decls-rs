use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (PartialEq , Encodable , Decodable , Debug)] pub struct SequenceRepetition { # [doc = " The sequence of token trees"] pub tts : Vec < TokenTree > , # [doc = " The optional separator"] pub separator : Option < Token > , # [doc = " Whether the sequence can be repeated zero (*), or one or more times (+)"] pub kleene : KleeneToken , # [doc = " The number of `Match`s that appear in the sequence (and subsequences)"] pub num_captures : usize , }