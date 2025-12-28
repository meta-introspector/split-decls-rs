macro_rules! deps {
    () => {
        Zip64CDELocatorBlock!();
        FixedSizeBlock!();
        ZipError!();
        Magic!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl FixedSizeBlock for Zip64CDELocatorBlock { const MAGIC : Magic = Magic :: ZIP64_CENTRAL_DIRECTORY_END_LOCATOR_SIGNATURE ; # [inline (always)] fn magic (self) -> Magic { self . magic } const WRONG_MAGIC_ERROR : ZipError = invalid ! ("Invalid zip64 locator digital signature header") ; to_and_from_le ! [(magic , Magic) , (disk_with_central_directory , u32) , (end_of_central_directory_offset , u64) , (number_of_disks , u32) ,] ; }
    };
}

impl_166!()