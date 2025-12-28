macro_rules! local_async_serial_core_with_return {
    () => {
        # [doc (hidden)] # [cfg (feature = "async")] pub async fn local_async_serial_core_with_return < R , E > (names : Vec < & str > , _path : Option < & str > , fut : impl std :: future :: Future < Output = Result < R , E > > + std :: marker :: Send ,) -> Result < R , E > { core_internal ! (names) ; fut . await }
    };
}

local_async_serial_core_with_return!();