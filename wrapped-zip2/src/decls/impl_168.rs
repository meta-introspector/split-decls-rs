macro_rules! deps {
    () => {
        Zip64CDELocatorBlock!();
        Zip64CentralDirectoryEndLocator!();
        ZipResult!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl Zip64CentralDirectoryEndLocator { pub fn parse < T : Read > (reader : & mut T) -> ZipResult < Zip64CentralDirectoryEndLocator > { let Zip64CDELocatorBlock { disk_with_central_directory , end_of_central_directory_offset , number_of_disks , .. } = Zip64CDELocatorBlock :: parse (reader) ? ; Ok (Zip64CentralDirectoryEndLocator { disk_with_central_directory , end_of_central_directory_offset , number_of_disks , }) } pub fn block (self) -> Zip64CDELocatorBlock { let Self { disk_with_central_directory , end_of_central_directory_offset , number_of_disks , } = self ; Zip64CDELocatorBlock { magic : Zip64CDELocatorBlock :: MAGIC , disk_with_central_directory , end_of_central_directory_offset , number_of_disks , } } pub fn write < T : Write > (self , writer : & mut T) -> ZipResult < () > { self . block () . write (writer) } }
    };
}

impl_168!();