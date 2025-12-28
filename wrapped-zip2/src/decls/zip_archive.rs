macro_rules! deps {
    () => {
        ZipFileData!();
    };
}

macro_rules! zip_archive {
    () => {
        deps!();
        pub (crate) mod zip_archive { use indexmap :: IndexMap ; use std :: sync :: Arc ; # [doc = " Extract immutable data from `ZipArchive` to make it cheap to clone"] # [derive (Debug)] pub (crate) struct Shared { pub (crate) files : IndexMap < Box < str > , super :: ZipFileData > , pub (super) offset : u64 , pub (super) dir_start : u64 , # [allow (dead_code)] pub (super) config : super :: Config , pub (crate) comment : Box < [u8] > , pub (crate) zip64_comment : Option < Box < [u8] > > , } # [derive (Debug)] pub (crate) struct SharedBuilder { pub (crate) files : Vec < super :: ZipFileData > , pub (super) offset : u64 , pub (super) dir_start : u64 , # [allow (dead_code)] pub (super) config : super :: Config , } impl SharedBuilder { pub fn build (self , comment : Box < [u8] > , zip64_comment : Option < Box < [u8] > >) -> Shared { let mut index_map = IndexMap :: with_capacity (self . files . len ()) ; self . files . into_iter () . for_each (| file | { index_map . insert (file . file_name . clone () , file) ; }) ; Shared { files : index_map , offset : self . offset , dir_start : self . dir_start , config : self . config , comment , zip64_comment , } } } # [doc = " ZIP archive reader"] # [doc = ""] # [doc = " At the moment, this type is cheap to clone if this is the case for the"] # [doc = " reader it uses. However, this is not guaranteed by this crate and it may"] # [doc = " change in the future."] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::io::prelude::*;"] # [doc = " fn list_zip_contents(reader: impl Read + Seek) -> zip::result::ZipResult<()> {"] # [doc = "     use zip::HasZipMetadata;"] # [doc = "     let mut zip = zip::ZipArchive::new(reader)?;"] # [doc = ""] # [doc = "     for i in 0..zip.len() {"] # [doc = "         let mut file = zip.by_index(i)?;"] # [doc = "         println!(\"Filename: {}\", file.name());"] # [doc = "         std::io::copy(&mut file, &mut std::io::stdout())?;"] # [doc = "     }"] # [doc = ""] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct ZipArchive < R > { pub (super) reader : R , pub (super) shared : Arc < Shared > , } }
    };
}

zip_archive!()