macro_rules! deps {
    () => {
        ActiveQueryGuard!();
    };
}

macro_rules! impl_489 {
    () => {
        deps!();
        impl Drop for ActiveQueryGuard < '_ > { fn drop (& mut self) { unsafe { self . local_state . with_query_stack_unchecked_mut (| stack | { stack . pop (self . database_key_index , # [cfg (debug_assertions)] self . push_len ,) ; }) } ; } }
    };
}

impl_489!()