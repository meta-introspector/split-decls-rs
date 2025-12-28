macro_rules! deps {
    () => {
        MachineSize!();
        Endian!();
    };
}

macro_rules! MachineInfo {
    () => {
        deps!();
        # [doc = " The properties of the target machine being compiled into."] # [derive (Clone , PartialEq , Eq , Serialize)] pub struct MachineInfo { pub endian : Endian , pub pointer_width : MachineSize , }
    };
}

MachineInfo!()