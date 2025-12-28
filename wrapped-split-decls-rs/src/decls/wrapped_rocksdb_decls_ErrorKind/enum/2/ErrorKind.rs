use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " RocksDB error kind."] # [derive (Debug , Clone , PartialEq , Eq)] pub enum ErrorKind { NotFound , Corruption , NotSupported , InvalidArgument , IOError , MergeInProgress , Incomplete , ShutdownInProgress , TimedOut , Aborted , Busy , Expired , TryAgain , CompactionTooLarge , ColumnFamilyDropped , Unknown , }