macro_rules! ring_buffer {
    () => {
        # [cfg (feature = "ringbuffer")] pub mod ring_buffer ;
    };
}

ring_buffer!();