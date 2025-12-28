use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn parse_errors (db : & dyn RootQueryDb , file_id : EditionedFileId) -> Option < & [SyntaxError] > { # [salsa_macros :: tracked (returns (ref))] fn parse_errors (db : & dyn RootQueryDb , file_id : EditionedFileId) -> Option < Box < [SyntaxError] > > { let errors = db . parse (file_id) . errors () ; match & * errors { [] => None , [..] => Some (errors . into ()) , } } parse_errors (db , file_id) . as_ref () . map (| it | & * * it) }