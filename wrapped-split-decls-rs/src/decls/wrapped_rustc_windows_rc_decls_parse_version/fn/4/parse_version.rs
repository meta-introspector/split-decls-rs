use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Parse a string in the format \"major.minor.patch\" into a [`ResourceVersion`]."] # [doc = " The build is set to 0."] # [doc = " Returns `None` if the version string is not in the expected format."] fn parse_version (version : & str) -> Option < ResourceVersion > { let mut parts = version . split ('.') ; let major = parts . next () ? . parse :: < u16 > () . ok () ? ; let minor = parts . next () ? . parse :: < u16 > () . ok () ? ; let patch = parts . next () ? . parse :: < u16 > () . ok () ? ; if parts . next () . is_some () { None } else { Some (ResourceVersion { major , minor , patch , build : 0 , }) } }
}