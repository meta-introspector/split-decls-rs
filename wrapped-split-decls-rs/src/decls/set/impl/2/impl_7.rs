use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : Eq + Hash > FromIterator < T > for SsoHashSet < T > { fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> SsoHashSet < T > { let mut set : SsoHashSet < T > = Default :: default () ; set . extend (iter) ; set } }