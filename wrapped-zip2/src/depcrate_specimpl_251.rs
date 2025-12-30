// Generated macro for impl_251 (impl)
macro_rules! Depcrate_specimpl_251 {
() => {
// Module: crate::spec
// Provides: {"impl_251"}
// Dependencies: {}
impl Zip64CentralDirectoryEndLocator { pub fn parse < T : Read > (reader : & mut T) -> ZipResult < Zip64CentralDirectoryEndLocator > { let Zip64CDELocatorBlock { disk_with_central_directory , end_of_central_directory_offset , number_of_disks , .. } = Zip64CDELocatorBlock :: parse (reader) ? ; Ok (Zip64CentralDirectoryEndLocator { disk_with_central_directory , end_of_central_directory_offset , number_of_disks , }) } pub fn block (self) -> Zip64CDELocatorBlock { let Self { disk_with_central_directory , end_of_central_directory_offset , number_of_disks , } = self ; Zip64CDELocatorBlock { magic : Zip64CDELocatorBlock :: MAGIC , disk_with_central_directory , end_of_central_directory_offset , number_of_disks , } } pub fn write < T : Write > (self , writer : & mut T) -> ZipResult < () > { self . block () . write (writer) } }
};
}
