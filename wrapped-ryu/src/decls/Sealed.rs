macro_rules! Sealed {
    () => {
        pub trait Sealed : Copy { fn is_nonfinite (self) -> bool ; fn format_nonfinite (self) -> & 'static str ; unsafe fn write_to_ryu_buffer (self , result : * mut u8) -> usize ; }
    };
}

Sealed!()