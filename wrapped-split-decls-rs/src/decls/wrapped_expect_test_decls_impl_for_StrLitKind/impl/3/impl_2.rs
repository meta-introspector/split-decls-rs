use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl StrLitKind { fn write_start (self , w : & mut impl std :: fmt :: Write) -> std :: fmt :: Result { match self { Self :: Normal => write ! (w , "\"") , Self :: Raw (n) => { write ! (w , "r") ? ; for _ in 0 .. n { write ! (w , "#") ? ; } write ! (w , "\"") } } } fn write_end (self , w : & mut impl std :: fmt :: Write) -> std :: fmt :: Result { match self { Self :: Normal => write ! (w , "\"") , Self :: Raw (n) => { write ! (w , "\"") ? ; for _ in 0 .. n { write ! (w , "#") ? ; } Ok (()) } } } }