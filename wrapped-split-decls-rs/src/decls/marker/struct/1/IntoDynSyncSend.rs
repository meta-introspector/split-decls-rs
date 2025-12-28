use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Copy , Clone)] pub struct IntoDynSyncSend < T : ? Sized + PointeeSized > (pub T) ;