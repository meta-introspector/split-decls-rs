// Generated macro for macro_16 (macro)
macro_rules! Depcrate_astmacro_16 {
() => {
// Module: crate::ast
// Provides: {"macro_16"}
// Dependencies: {}
tokenizable ! { # [doc = " Reserved words that have special meaning to the shell. They are used to"] # [doc = " begin and end the shell's compound commands."] # [doc = ""] # [doc = " Note that [bash](https://www.gnu.org/software/bash/manual/bash.html#Reserved-Words)"] # [doc = " also considers `!`, `[[`, `]]`, `{` and `}` to be keywords, but because"] # [doc = " their parsing depends on where they appear, we don't include them here."] enum Keyword { Case ("case") , Coproc ("coproc") , Do ("do") , Done ("done") , Elif ("elif") , Else ("else") , Esac ("esac") , Fi ("fi") , For ("for") , Function ("function") , If ("if") , In ("in") , Select ("select") , Then ("then") , Until ("until") , While ("while") , } }
};
}
