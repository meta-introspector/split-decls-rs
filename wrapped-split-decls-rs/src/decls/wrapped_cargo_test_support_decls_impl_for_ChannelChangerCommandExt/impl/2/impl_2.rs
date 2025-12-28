use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl ChannelChangerCommandExt for snapbox :: cmd :: Command { fn masquerade_as_nightly_cargo (self , _reasons : & [& str]) -> Self { self . env ("__CARGO_TEST_CHANNEL_OVERRIDE_DO_NOT_USE_THIS" , "nightly") } }