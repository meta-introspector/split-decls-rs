macro_rules! FileLoader {
    () => {
        # [doc = " An abstraction over the fs operations used by the Parser."] pub trait FileLoader { # [doc = " Query the existence of a file."] fn file_exists (& self , path : & Path) -> bool ; # [doc = " Read the contents of a UTF-8 file into memory."] # [doc = " This function must return a String because we normalize"] # [doc = " source files, which may require resizing."] fn read_file (& self , path : & Path) -> io :: Result < String > ; # [doc = " Read the contents of a potentially non-UTF-8 file into memory."] # [doc = " We don't normalize binary files, so we can start in an Arc."] fn read_binary_file (& self , path : & Path) -> io :: Result < Arc < [u8] > > ; }
    };
}

FileLoader!();