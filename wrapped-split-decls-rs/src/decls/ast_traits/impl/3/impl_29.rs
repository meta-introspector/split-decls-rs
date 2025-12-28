use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < Wrapped , Tag > AstNodeWrapper < Wrapped , Tag > { pub fn new (wrapped : Wrapped , _tag : Tag) -> AstNodeWrapper < Wrapped , Tag > { AstNodeWrapper { wrapped , tag : Default :: default () } } pub fn from_mut (wrapped : & mut Wrapped , _tag : Tag) -> & mut AstNodeWrapper < Wrapped , Tag > { unsafe { & mut * < * mut Wrapped > :: cast (wrapped) } } }