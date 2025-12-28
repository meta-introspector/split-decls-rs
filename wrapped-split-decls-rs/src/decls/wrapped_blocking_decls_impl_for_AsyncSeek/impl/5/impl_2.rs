use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : Seek + Send + 'static > AsyncSeek for Unblock < T > { fn poll_seek (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , pos : SeekFrom ,) -> Poll < io :: Result < u64 > > { loop { match & mut self . state { State :: WithMut (..) | State :: Streaming (..) | State :: Reading (..) | State :: Writing (..) => { ready ! (self . poll_stop (cx)) ? ; } State :: Idle (io) => { let mut io = io . take () . expect ("inner value was taken out") ; let task = Executor :: spawn (async move { let res = io . seek (pos) ; (pos , res , io) }) ; self . state = State :: Seeking (task) ; } State :: Seeking (task) => { let (original_pos , res , io) = ready ! (Pin :: new (task) . poll (cx)) ; self . state = State :: Idle (Some (io)) ; let current = res ? ; if original_pos == pos { return Poll :: Ready (Ok (current)) ; } } } } } }
}