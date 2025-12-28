macro_rules! deps {
    () => {
        Endian!();
        MachineInfo!();
        MachineSize!();
    };
}

macro_rules! impl_294 {
    () => {
        deps!();
        impl MachineInfo { pub fn target () -> MachineInfo { with (| cx | cx . target_info ()) } pub fn target_endianness () -> Endian { with (| cx | cx . target_info () . endian) } pub fn target_pointer_width () -> MachineSize { with (| cx | cx . target_info () . pointer_width) } }
    };
}

impl_294!()