use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Copy , Clone)] pub struct IntoDynSyncSend < T : ? Sized + PointeeSized > (pub T) ;
}