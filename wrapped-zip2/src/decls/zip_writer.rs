macro_rules! deps {
    () => {
        GenericZipWriter!();
        ZipFileData!();
        ZipWriterStats!();
    };
}

macro_rules! zip_writer {
    () => {
        deps!();
        pub (crate) mod zip_writer { use super :: * ; # [doc = " ZIP archive generator"] # [doc = ""] # [doc = " Handles the bookkeeping involved in building an archive, and provides an"] # [doc = " API to edit its contents."] # [doc = ""] # [doc = " ```"] # [doc = " # fn doit() -> zip::result::ZipResult<()>"] # [doc = " # {"] # [doc = " # use zip::ZipWriter;"] # [doc = " use std::io::Write;"] # [doc = " use zip::write::SimpleFileOptions;"] # [doc = ""] # [doc = " // We use a cursor + vec here, though you'd normally use a `File`"] # [doc = " let mut cur = std::io::Cursor::new(Vec::new());"] # [doc = " let mut zip = ZipWriter::new(&mut cur);"] # [doc = ""] # [doc = " let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);"] # [doc = " zip.start_file(\"hello_world.txt\", options)?;"] # [doc = " zip.write(b\"Hello, World!\")?;"] # [doc = ""] # [doc = " // Apply the changes you've made."] # [doc = " // Dropping the `ZipWriter` will have the same effect, but may silently fail"] # [doc = " zip.finish()?;"] # [doc = ""] # [doc = " // raw zip data is available as a Vec<u8>"] # [doc = " let zip_bytes = cur.into_inner();"] # [doc = ""] # [doc = " # Ok(())"] # [doc = " # }"] # [doc = " # doit().unwrap();"] # [doc = " ```"] pub struct ZipWriter < W : Write + Seek > { pub (super) inner : GenericZipWriter < W > , pub (super) files : IndexMap < Box < str > , ZipFileData > , pub (super) stats : ZipWriterStats , pub (super) writing_to_file : bool , pub (super) writing_raw : bool , pub (super) comment : Box < [u8] > , pub (super) zip64_comment : Option < Box < [u8] > > , pub (super) flush_on_finish_file : bool , pub (super) seek_possible : bool , } impl < W : Write + Seek > Debug for ZipWriter < W > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { f . write_fmt (format_args ! ("ZipWriter {{files: {:?}, stats: {:?}, writing_to_file: {}, writing_raw: {}, comment: {:?}, flush_on_finish_file: {}}}" , self . files , self . stats , self . writing_to_file , self . writing_raw , self . comment , self . flush_on_finish_file)) } } }
    };
}

zip_writer!()