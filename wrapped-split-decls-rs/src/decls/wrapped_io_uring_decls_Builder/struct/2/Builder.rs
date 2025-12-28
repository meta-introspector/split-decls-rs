use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " IoUring build params"] # [derive (Clone , Default)] pub struct Builder < S = squeue :: Entry , C = cqueue :: Entry > where S : squeue :: EntryMarker , C : cqueue :: EntryMarker , { dontfork : bool , params : sys :: io_uring_params , phantom : PhantomData < (S , C) > , }
}