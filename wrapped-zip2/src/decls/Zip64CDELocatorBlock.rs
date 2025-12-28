macro_rules! deps {
    () => {
        Magic!();
    };
}

macro_rules! Zip64CDELocatorBlock {
    () => {
        deps!();
        # [derive (Copy , Clone)] # [repr (packed , C)] pub (crate) struct Zip64CDELocatorBlock { magic : Magic , pub disk_with_central_directory : u32 , pub end_of_central_directory_offset : u64 , pub number_of_disks : u32 , }
    };
}

Zip64CDELocatorBlock!()