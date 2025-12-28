use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl std :: fmt :: Debug for Counters { fn fmt (& self , fmt : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let word = format ! ("{:016x}" , self . word) ; fmt . debug_struct ("Counters") . field ("word" , & word) . field ("jobs" , & self . jobs_counter () . 0) . field ("inactive" , & self . inactive_threads ()) . field ("sleeping" , & self . sleeping_threads ()) . finish () } }
}