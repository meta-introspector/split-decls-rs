// Generated macro for impl_79 (impl)
macro_rules! Depcrate_customimpl_79 {
() => {
// Module: crate::custom
// Provides: {"impl_79"}
// Dependencies: {}
impl Language { pub fn lookup (runner : & Runner < '_ > , language : & str) -> Result < Language > { for (ext , script) in runner . opts . custom . custom . iter () { if ext == language { return Ok (Language { extension : ext . to_string () , script : script . to_string () , }) ; } } bail ! ("file extension `{language}` is unknown, but you can pass \
             a script with `--custom {language}=my-script.sh` to get it working") } }
};
}
