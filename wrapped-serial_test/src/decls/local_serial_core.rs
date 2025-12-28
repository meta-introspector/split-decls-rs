macro_rules! local_serial_core {
    () => {
        # [doc (hidden)] pub fn local_serial_core (names : Vec < & str > , _path : Option < & str > , function : fn ()) { core_internal ! (names) ; function () ; }
    };
}

local_serial_core!()