// Generated macro for build_cargo (function)
macro_rules! Depcratebuild_cargo {
() => {
// Module: crate
// Provides: {"build_cargo"}
// Dependencies: {}
# [doc = " Builds the man pages for Cargo."] # [doc = ""] # [doc = " The source for the man pages are located in src/doc/man/ in markdown format."] # [doc = " These also are handlebars templates, see crates/mdman/README.md for details."] # [doc = ""] # [doc = " The generated man pages are placed in the src/etc/man/ directory. The pages"] # [doc = " are also expanded into markdown (after being expanded by handlebars) and"] # [doc = " saved in the src/doc/src/commands/ directory. These are included in the"] # [doc = " Cargo book, which is converted to HTML by mdbook."] fn build_cargo () -> io :: Result < () > { let src_paths = { let mut src_paths = Vec :: new () ; for entry in fs :: read_dir ("src/doc/man") ? { let entry = entry ? ; let file_name = entry . file_name () ; let file_name = file_name . to_str () . unwrap () ; if file_name . starts_with ("cargo") && file_name . ends_with (".md") { src_paths . push (entry . path ()) ; } } src_paths } ; let outs = [("md" , "src/doc/src/commands") , ("txt" , "src/doc/man/generated_txt") , ("man" , "src/etc/man") ,] ; let args = ["--url" , "https://doc.rust-lang.org/cargo/commands/" , "--man" , "rustc:1=https://doc.rust-lang.org/rustc/index.html" , "--man" , "rustdoc:1=https://doc.rust-lang.org/rustdoc/index.html" ,] ; build_man ("cargo" , & src_paths [..] , & outs , & args) }
};
}
