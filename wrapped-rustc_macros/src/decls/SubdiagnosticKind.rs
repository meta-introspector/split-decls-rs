macro_rules! deps {
    () => {
        Applicability!();
        SpannedOption!();
        SuggestionKind!();
    };
}

macro_rules! SubdiagnosticKind {
    () => {
        deps!();
        # [doc = " Types of subdiagnostics that can be created using attributes"] # [derive (Clone)] pub (super) enum SubdiagnosticKind { # [doc = " `#[label(...)]`"] Label , # [doc = " `#[note(...)]`"] Note , # [doc = " `#[note_once(...)]`"] NoteOnce , # [doc = " `#[help(...)]`"] Help , # [doc = " `#[help_once(...)]`"] HelpOnce , # [doc = " `#[warning(...)]`"] Warn , # [doc = " `#[suggestion{,_short,_hidden,_verbose}]`"] Suggestion { suggestion_kind : SuggestionKind , applicability : SpannedOption < Applicability > , # [doc = " Identifier for variable used for formatted code, e.g. `___code_0`. Enables separation"] # [doc = " of formatting and diagnostic emission so that `arg` calls can happen in-between.."] code_field : syn :: Ident , # [doc = " Initialization logic for `code_field`'s variable, e.g."] # [doc = " `let __formatted_code = /* whatever */;`"] code_init : TokenStream , } , # [doc = " `#[multipart_suggestion{,_short,_hidden,_verbose}]`"] MultipartSuggestion { suggestion_kind : SuggestionKind , applicability : SpannedOption < Applicability > , } , }
    };
}

SubdiagnosticKind!()