use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Tests `bytepos_to_file_charpos`."] # [test] fn t4 () { let sm = init_source_map () ; let cp1 = sm . bytepos_to_file_charpos (BytePos (22)) ; assert_eq ! (cp1 , CharPos (22)) ; let cp2 = sm . bytepos_to_file_charpos (BytePos (25)) ; assert_eq ! (cp2 , CharPos (0)) ; }