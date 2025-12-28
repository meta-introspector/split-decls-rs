macro_rules! deps {
    () => {
        HANDLE!();
        HEAP_FLAGS!();
    };
}

macro_rules! macro_82 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn HeapAlloc (hheap : HANDLE , dwflags : HEAP_FLAGS , dwbytes : usize) -> * mut core :: ffi :: c_void) ;
    };
}

macro_82!()