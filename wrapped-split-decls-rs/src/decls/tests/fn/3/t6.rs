use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Tests `bytepos_to_file_charpos` in the presence of multi-byte chars."] # [test] fn t6 () { let sm = init_source_map_mbc () ; let cp1 = sm . bytepos_to_file_charpos (BytePos (3)) ; assert_eq ! (cp1 , CharPos (3)) ; let cp2 = sm . bytepos_to_file_charpos (BytePos (6)) ; assert_eq ! (cp2 , CharPos (4)) ; let cp3 = sm . bytepos_to_file_charpos (BytePos (56)) ; assert_eq ! (cp3 , CharPos (12)) ; let cp4 = sm . bytepos_to_file_charpos (BytePos (61)) ; assert_eq ! (cp4 , CharPos (15)) ; }