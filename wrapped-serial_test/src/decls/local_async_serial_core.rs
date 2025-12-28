macro_rules! local_async_serial_core {
    () => {
        # [doc (hidden)] # [cfg (feature = "async")] pub async fn local_async_serial_core (names : Vec < & str > , _path : Option < & str > , fut : impl std :: future :: Future < Output = () > ,) { core_internal ! (names) ; fut . await ; }
    };
}

local_async_serial_core!();