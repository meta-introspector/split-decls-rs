// Generated macro for impl_14 (impl)
macro_rules! Depcrate_readerimpl_14 {
() => {
// Module: crate::reader
// Provides: {"impl_14"}
// Dependencies: {}
# [doc = " Avoids writing the value or position to avoid creating a side channel,"] # [doc = " though `Reader` can't avoid leaking the position via timing."] impl core :: fmt :: Debug for Reader < '_ > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_struct ("Reader") . finish () } }
};
}
