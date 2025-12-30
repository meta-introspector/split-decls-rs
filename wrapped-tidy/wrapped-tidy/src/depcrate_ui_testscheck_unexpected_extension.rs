// Generated macro for check_unexpected_extension (function)
macro_rules! Depcrate_ui_testscheck_unexpected_extension {
() => {
// Module: crate::ui_tests
// Provides: {"check_unexpected_extension"}
// Dependencies: {}
fn check_unexpected_extension (bad : & mut bool , file_path : & Path , ext : & str) { const EXPECTED_TEST_FILE_EXTENSIONS : & [& str] = & ["rs" , "stderr" , "svg" , "stdout" , "fixed" , "md" , "ftl" ,] ; const EXTENSION_EXCEPTION_PATHS : & [& str] = & ["tests/ui/asm/named-asm-labels.s" , "tests/ui/codegen/mismatched-data-layout.json" , "tests/ui/check-cfg/my-awesome-platform.json" , "tests/ui/argfile/commandline-argfile-badutf8.args" , "tests/ui/argfile/commandline-argfile.args" , "tests/ui/crate-loading/auxiliary/libfoo.rlib" , "tests/ui/include-macros/data.bin" , "tests/ui/include-macros/file.txt" , "tests/ui/macros/macro-expanded-include/file.txt" , "tests/ui/macros/not-utf8.bin" , "tests/ui/macros/syntax-extension-source-utils-files/includeme.fragment" , "tests/ui/proc-macro/auxiliary/included-file.txt" , "tests/ui/unpretty/auxiliary/data.txt" , "tests/ui/invalid/foo.natvis.xml" , "tests/ui/sanitizer/dataflow-abilist.txt" , "tests/ui/shell-argfiles/shell-argfiles.args" , "tests/ui/shell-argfiles/shell-argfiles-badquotes.args" , "tests/ui/shell-argfiles/shell-argfiles-via-argfile-shell.args" , "tests/ui/shell-argfiles/shell-argfiles-via-argfile.args" , "tests/ui/std/windows-bat-args1.bat" , "tests/ui/std/windows-bat-args2.bat" , "tests/ui/std/windows-bat-args3.bat" ,] ; if ! (EXPECTED_TEST_FILE_EXTENSIONS . contains (& ext) || EXTENSION_EXCEPTION_PATHS . iter () . any (| path | file_path . ends_with (path))) { tidy_error ! (bad , "file {} has unexpected extension {}" , file_path . display () , ext) ; } }
};
}
