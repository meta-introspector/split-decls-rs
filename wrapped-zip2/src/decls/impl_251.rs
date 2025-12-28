macro_rules! deps {
    () => {
        ZipResult!();
        MaybeEncrypted!();
    };
}

macro_rules! impl_251 {
    () => {
        deps!();
        impl < A : Read + Write + Seek > ZipWriter < A > { # [doc = " Initializes the archive from an existing ZIP archive, making it ready for append."] # [doc = ""] # [doc = " This uses a default configuration to initially read the archive."] pub fn new_append (readwriter : A) -> ZipResult < ZipWriter < A > > { Self :: new_append_with_config (Default :: default () , readwriter) } # [doc = " Initializes the archive from an existing ZIP archive, making it ready for append."] # [doc = ""] # [doc = " This uses the given read configuration to initially read the archive."] pub fn new_append_with_config (config : Config , mut readwriter : A) -> ZipResult < ZipWriter < A > > { readwriter . seek (SeekFrom :: Start (0)) ? ; let shared = ZipArchive :: get_metadata (config , & mut readwriter) ? ; Ok (ZipWriter { inner : Storer (MaybeEncrypted :: Unencrypted (readwriter)) , files : shared . files , stats : Default :: default () , writing_to_file : false , comment : shared . comment , zip64_comment : shared . zip64_comment , writing_raw : true , flush_on_finish_file : false , seek_possible : true , }) } # [doc = " `flush_on_finish_file` is designed to support a streaming `inner` that may unload flushed"] # [doc = " bytes. It flushes a file's header and body once it starts writing another file. A ZipWriter"] # [doc = " will not try to seek back into where a previous file was written unless"] # [doc = " either [`ZipWriter::abort_file`] is called while [`ZipWriter::is_writing_file`] returns"] # [doc = " false, or [`ZipWriter::deep_copy_file`] is called. In the latter case, it will only need to"] # [doc = " read previously-written files and not overwrite them."] # [doc = ""] # [doc = " Note: when using an `inner` that cannot overwrite flushed bytes, do not wrap it in a"] # [doc = " [BufWriter], because that has a [Seek::seek] method that implicitly calls"] # [doc = " [BufWriter::flush], and ZipWriter needs to seek backward to update each file's header with"] # [doc = " the size and checksum after writing the body."] # [doc = ""] # [doc = " This setting is false by default."] pub fn set_flush_on_finish_file (& mut self , flush_on_finish_file : bool) { self . flush_on_finish_file = flush_on_finish_file ; } }
    };
}

impl_251!();