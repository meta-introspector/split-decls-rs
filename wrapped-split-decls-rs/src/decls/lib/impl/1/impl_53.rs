use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl FnContext { # [inline] fn new (migrated : bool) -> Self { FnContext { migrated , _marker : PhantomData } } }