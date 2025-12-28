use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn scan_output2_blocks () -> Result < HashMap < String , String > > { let mut blocks = HashMap :: new () ; if Path :: new ("output2") . exists () { for entry in fs :: read_dir ("output2") ? { let entry = entry ? ; if entry . path () . extension () . map_or (false , | ext | ext == "rs") { let content = fs :: read_to_string (entry . path ()) ? ; let filename = entry . file_name () . to_string_lossy () . to_string () ; blocks . insert (filename , content) ; } } } Ok (blocks) }