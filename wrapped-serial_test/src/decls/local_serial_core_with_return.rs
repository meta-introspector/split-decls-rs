macro_rules! local_serial_core_with_return {
    () => {
        # [doc (hidden)] pub fn local_serial_core_with_return < R , E > (names : Vec < & str > , _path : Option < String > , function : fn () -> Result < R , E > ,) -> Result < R , E > { core_internal ! (names) ; function () }
    };
}

local_serial_core_with_return!();