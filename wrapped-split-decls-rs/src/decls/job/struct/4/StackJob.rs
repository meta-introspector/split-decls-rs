use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A job that will be owned by a stack slot. This means that when it"] # [doc = " executes it need not free any heap data, the cleanup occurs when"] # [doc = " the stack frame is later popped. The function parameter indicates"] # [doc = " `true` if the job was stolen -- executed on a different thread."] pub (super) struct StackJob < L , F , R > where L : Latch + Sync , F : FnOnce (bool) -> R + Send , R : Send , { pub (super) latch : L , func : UnsafeCell < Option < F > > , result : UnsafeCell < JobResult < R > > , tlv : Tlv , }
}