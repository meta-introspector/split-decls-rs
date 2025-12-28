use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl FromStr for SourceFileHashAlgorithm { type Err = () ; fn from_str (s : & str) -> Result < SourceFileHashAlgorithm , () > { match s { "md5" => Ok (SourceFileHashAlgorithm :: Md5) , "sha1" => Ok (SourceFileHashAlgorithm :: Sha1) , "sha256" => Ok (SourceFileHashAlgorithm :: Sha256) , "blake3" => Ok (SourceFileHashAlgorithm :: Blake3) , _ => Err (()) , } } }