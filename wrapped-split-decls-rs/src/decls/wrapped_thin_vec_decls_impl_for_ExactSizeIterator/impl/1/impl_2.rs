use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < I : Iterator > ExactSizeIterator for Splice < '_ , I > { }