use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn basic () { let mut map = SnapshotMap :: default () ; map . insert (22 , "twenty-two") ; let snapshot = map . snapshot () ; map . insert (22 , "thirty-three") ; assert_eq ! (map [& 22] , "thirty-three") ; map . insert (44 , "forty-four") ; assert_eq ! (map [& 44] , "forty-four") ; assert_eq ! (map . get (& 33) , None) ; map . rollback_to (snapshot) ; assert_eq ! (map [& 22] , "twenty-two") ; assert_eq ! (map . get (& 33) , None) ; assert_eq ! (map . get (& 44) , None) ; }