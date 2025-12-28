macro_rules! get_thread_id {
    () => {
        fn get_thread_id () -> u32 { std :: thread :: current () . id () . as_u64 () . get () as u32 }
    };
}

get_thread_id!();