use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const BASE_64 : [ascii :: Char ; MAX_BASE] = { let bytes = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ@$" ; let Some (ascii) = bytes . as_ascii () else { panic ! () } ; * ascii } ;