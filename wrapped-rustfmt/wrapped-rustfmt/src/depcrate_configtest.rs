// Generated macro for test (module)
macro_rules! Depcrate_configtest {
() => {
// Module: crate::config
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use std :: str ; use crate :: config :: macro_names :: { MacroName , MacroSelectors } ; use rustfmt_config_proc_macro :: { nightly_only_test , stable_only_test } ; # [allow (dead_code)] mod mock { use super :: super :: * ; use crate :: config_option_with_style_edition_default ; use rustfmt_config_proc_macro :: config_type ; # [config_type] pub (crate) enum PartiallyUnstableOption { V1 , V2 , # [unstable_variant] V3 , } config_option_with_style_edition_default ! (StableOption , bool , _ => false ; UnstableOption , bool , _ => false ; PartiallyUnstable , PartiallyUnstableOption , _ => PartiallyUnstableOption :: V1 ;) ; create_config ! { max_width : MaxWidth , true , "Maximum width of each line" ; required_version : RequiredVersion , false , "Require a specific version of rustfmt." ; ignore : Ignore , false , "Skip formatting the specified files and directories." ; verbose : Verbose , false , "How much to information to emit to the user" ; file_lines : FileLinesConfig , false , "Lines to format; this is not supported in rustfmt.toml, and can only be specified \
                    via the --file-lines option" ; imports_granularity : ImportsGranularityConfig , false , "Merge imports" ; merge_imports : MergeImports , false , "(deprecated: use imports_granularity instead)" ; fn_args_layout : FnArgsLayout , true , "(deprecated: use fn_params_layout instead)" ; fn_params_layout : FnParamsLayout , true , "Control the layout of parameters in a function signatures." ; hide_parse_errors : HideParseErrors , false , "(deprecated: use show_parse_errors instead)" ; show_parse_errors : ShowParseErrors , false , "Show errors from the parser (unstable)" ; use_small_heuristics : UseSmallHeuristics , true , "Whether to use different formatting for items and \
                 expressions if they satisfy a heuristic notion of 'small'." ; width_heuristics : WidthHeuristicsConfig , false , "'small' heuristic values" ; fn_call_width : FnCallWidth , true , "Maximum width of the args of a function call before \
                falling back to vertical formatting." ; attr_fn_like_width : AttrFnLikeWidth , true , "Maximum width of the args of a \
                function-like attributes before falling back to vertical formatting." ; struct_lit_width : StructLitWidth , true , "Maximum width in the body of a struct lit \
                before falling back to vertical formatting." ; struct_variant_width : StructVariantWidth , true , "Maximum width in the body of a struct \
                variant before falling back to vertical formatting." ; array_width : ArrayWidth , true , "Maximum width of an array literal before falling \
                back to vertical formatting." ; chain_width : ChainWidth , true , "Maximum length of a chain to fit on a single line." ; single_line_if_else_max_width : SingleLineIfElseMaxWidth , true , "Maximum line length \
                for single line if-else expressions. A value of zero means always break if-else \
                expressions." ; single_line_let_else_max_width : SingleLineLetElseMaxWidth , false , "Maximum line length \
                for single line let-else statements. A value of zero means always format the \
                divergent `else` block over multiple lines." ; stable_option : StableOption , true , "A stable option" ; unstable_option : UnstableOption , false , "An unstable option" ; partially_unstable_option : PartiallyUnstable , true , "A partially unstable option" ; edition : EditionConfig , true , "blah" ; style_edition : StyleEditionConfig , true , "blah" ; version : VersionConfig , false , "blah blah" } # [cfg (test)] mod partially_unstable_option { use super :: { Config , PartialConfig , PartiallyUnstableOption } ; use rustfmt_config_proc_macro :: { nightly_only_test , stable_only_test } ; use std :: path :: Path ; # [doc = " From the config file, we can fill with a stable variant"] # [test] fn test_from_toml_stable_value () { let toml = r#"
                    partially_unstable_option = "V2"
                "# ; let partial_config : PartialConfig = toml :: from_str (toml) . unwrap () ; let config = Config :: default () ; let config = config . fill_from_parsed_config (partial_config , Path :: new ("")) ; assert_eq ! (config . partially_unstable_option () , PartiallyUnstableOption :: V2) ; } # [doc = " From the config file, we cannot fill with an unstable variant (stable only)"] # [stable_only_test] # [test] fn test_from_toml_unstable_value_on_stable () { let toml = r#"
                    partially_unstable_option = "V3"
                "# ; let partial_config : PartialConfig = toml :: from_str (toml) . unwrap () ; let config = Config :: default () ; let config = config . fill_from_parsed_config (partial_config , Path :: new ("")) ; assert_eq ! (config . partially_unstable_option () , PartiallyUnstableOption :: V1) ; } # [doc = " From the config file, we can fill with an unstable variant (nightly only)"] # [nightly_only_test] # [test] fn test_from_toml_unstable_value_on_nightly () { let toml = r#"
                    partially_unstable_option = "V3"
                "# ; let partial_config : PartialConfig = toml :: from_str (toml) . unwrap () ; let config = Config :: default () ; let config = config . fill_from_parsed_config (partial_config , Path :: new ("")) ; assert_eq ! (config . partially_unstable_option () , PartiallyUnstableOption :: V3) ; } } } # [test] fn test_config_set () { let mut config = Config :: default () ; config . set () . verbose (Verbosity :: Quiet) ; assert_eq ! (config . verbose () , Verbosity :: Quiet) ; config . set () . verbose (Verbosity :: Normal) ; assert_eq ! (config . verbose () , Verbosity :: Normal) ; } # [test] fn test_config_used_to_toml () { let config = Config :: default () ; let merge_derives = config . merge_derives () ; let skip_children = config . skip_children () ; let used_options = config . used_options () ; let toml = used_options . to_toml () . unwrap () ; assert_eq ! (toml , format ! ("merge_derives = {merge_derives}\nskip_children = {skip_children}\n" ,)) ; } # [test] fn test_was_set () { let config = Config :: from_toml ("hard_tabs = true" , Path :: new ("./rustfmt.toml")) . unwrap () ; assert_eq ! (config . was_set () . hard_tabs () , true) ; assert_eq ! (config . was_set () . verbose () , false) ; } const PRINT_DOCS_STABLE_OPTION : & str = "stable_option <boolean> Default: false" ; const PRINT_DOCS_UNSTABLE_OPTION : & str = "unstable_option <boolean> Default: false (unstable)" ; const PRINT_DOCS_PARTIALLY_UNSTABLE_OPTION : & str = "partially_unstable_option [V1|V2|V3 (unstable)] Default: V1" ; # [test] fn test_print_docs_exclude_unstable () { use self :: mock :: Config ; let mut output = Vec :: new () ; Config :: print_docs (& mut output , false) ; let s = str :: from_utf8 (& output) . unwrap () ; assert_eq ! (s . contains (PRINT_DOCS_STABLE_OPTION) , true) ; assert_eq ! (s . contains (PRINT_DOCS_UNSTABLE_OPTION) , false) ; assert_eq ! (s . contains (PRINT_DOCS_PARTIALLY_UNSTABLE_OPTION) , true) ; } # [test] fn test_print_docs_include_unstable () { use self :: mock :: Config ; let mut output = Vec :: new () ; Config :: print_docs (& mut output , true) ; let s = str :: from_utf8 (& output) . unwrap () ; assert_eq ! (s . contains (PRINT_DOCS_STABLE_OPTION) , true) ; assert_eq ! (s . contains (PRINT_DOCS_UNSTABLE_OPTION) , true) ; assert_eq ! (s . contains (PRINT_DOCS_PARTIALLY_UNSTABLE_OPTION) , true) ; } # [test] fn test_dump_default_config () { let default_config = format ! (r#"max_width = 100
hard_tabs = false
tab_spaces = 4
newline_style = "Auto"
indent_style = "Block"
use_small_heuristics = "Default"
fn_call_width = 60
attr_fn_like_width = 70
struct_lit_width = 18
struct_variant_width = 35
array_width = 60
chain_width = 60
single_line_if_else_max_width = 50
single_line_let_else_max_width = 50
wrap_comments = false
format_code_in_doc_comments = false
doc_comment_code_block_width = 100
comment_width = 80
normalize_comments = false
normalize_doc_attributes = false
format_strings = false
format_macro_matchers = false
format_macro_bodies = true
skip_macro_invocations = []
hex_literal_case = "Preserve"
float_literal_trailing_zero = "Preserve"
empty_item_single_line = true
struct_lit_single_line = true
fn_single_line = false
where_single_line = false
imports_indent = "Block"
imports_layout = "Mixed"
imports_granularity = "Preserve"
group_imports = "Preserve"
reorder_imports = true
reorder_modules = true
reorder_impl_items = false
type_punctuation_density = "Wide"
space_before_colon = false
space_after_colon = true
spaces_around_ranges = false
binop_separator = "Front"
remove_nested_parens = true
combine_control_expr = true
short_array_element_width_threshold = 10
overflow_delimited_expr = false
struct_field_align_threshold = 0
enum_discrim_align_threshold = 0
match_arm_blocks = true
match_arm_leading_pipes = "Never"
match_arm_indent = true
force_multiline_blocks = false
fn_params_layout = "Tall"
brace_style = "SameLineWhere"
control_brace_style = "AlwaysSameLine"
trailing_semicolon = true
trailing_comma = "Vertical"
match_block_trailing_comma = false
blank_lines_upper_bound = 1
blank_lines_lower_bound = 0
edition = "2015"
style_edition = "2015"
version = "One"
inline_attribute_width = 0
format_generated_files = true
generated_marker_line_search_limit = 5
merge_derives = true
use_try_shorthand = false
use_field_init_shorthand = false
force_explicit_abi = true
condense_wildcard_suffixes = false
color = "Auto"
required_version = "{}"
unstable_features = false
disable_all_formatting = false
skip_children = false
show_parse_errors = true
error_on_line_overflow = false
error_on_unformatted = false
ignore = []
emit_mode = "Files"
make_backup = false
"# , env ! ("CARGO_PKG_VERSION")) ; let toml = Config :: default () . all_options () . to_toml () . unwrap () ; assert_eq ! (& toml , & default_config) ; } # [test] fn test_dump_style_edition_2024_config () { let edition_2024_config = format ! (r#"max_width = 100
hard_tabs = false
tab_spaces = 4
newline_style = "Auto"
indent_style = "Block"
use_small_heuristics = "Default"
fn_call_width = 60
attr_fn_like_width = 70
struct_lit_width = 18
struct_variant_width = 35
array_width = 60
chain_width = 60
single_line_if_else_max_width = 50
single_line_let_else_max_width = 50
wrap_comments = false
format_code_in_doc_comments = false
doc_comment_code_block_width = 100
comment_width = 80
normalize_comments = false
normalize_doc_attributes = false
format_strings = false
format_macro_matchers = false
format_macro_bodies = true
skip_macro_invocations = []
hex_literal_case = "Preserve"
float_literal_trailing_zero = "Preserve"
empty_item_single_line = true
struct_lit_single_line = true
fn_single_line = false
where_single_line = false
imports_indent = "Block"
imports_layout = "Mixed"
imports_granularity = "Preserve"
group_imports = "Preserve"
reorder_imports = true
reorder_modules = true
reorder_impl_items = false
type_punctuation_density = "Wide"
space_before_colon = false
space_after_colon = true
spaces_around_ranges = false
binop_separator = "Front"
remove_nested_parens = true
combine_control_expr = true
short_array_element_width_threshold = 10
overflow_delimited_expr = false
struct_field_align_threshold = 0
enum_discrim_align_threshold = 0
match_arm_blocks = true
match_arm_leading_pipes = "Never"
match_arm_indent = true
force_multiline_blocks = false
fn_params_layout = "Tall"
brace_style = "SameLineWhere"
control_brace_style = "AlwaysSameLine"
trailing_semicolon = true
trailing_comma = "Vertical"
match_block_trailing_comma = false
blank_lines_upper_bound = 1
blank_lines_lower_bound = 0
edition = "2015"
style_edition = "2024"
version = "Two"
inline_attribute_width = 0
format_generated_files = true
generated_marker_line_search_limit = 5
merge_derives = true
use_try_shorthand = false
use_field_init_shorthand = false
force_explicit_abi = true
condense_wildcard_suffixes = false
color = "Auto"
required_version = "{}"
unstable_features = false
disable_all_formatting = false
skip_children = false
show_parse_errors = true
error_on_line_overflow = false
error_on_unformatted = false
ignore = []
emit_mode = "Files"
make_backup = false
"# , env ! ("CARGO_PKG_VERSION")) ; let toml = Config :: default_with_style_edition (StyleEdition :: Edition2024) . all_options () . to_toml () . unwrap () ; assert_eq ! (& toml , & edition_2024_config) ; } # [test] fn test_editions_2015_2018_2021_identical () { let get_edition_toml = | style_edition : StyleEdition | { Config :: default_with_style_edition (style_edition) . all_options () . to_toml () . unwrap () } ; let edition2015 = get_edition_toml (StyleEdition :: Edition2015) ; let edition2018 = get_edition_toml (StyleEdition :: Edition2018) ; let edition2021 = get_edition_toml (StyleEdition :: Edition2021) ; assert_eq ! (edition2015 , edition2018) ; assert_eq ! (edition2018 , edition2021) ; } # [stable_only_test] # [test] fn test_as_not_nightly_channel () { let mut config = Config :: default () ; assert_eq ! (config . was_set () . unstable_features () , false) ; config . set () . unstable_features (true) ; assert_eq ! (config . was_set () . unstable_features () , false) ; } # [nightly_only_test] # [test] fn test_as_nightly_channel () { let mut config = Config :: default () ; config . set () . unstable_features (true) ; assert_eq ! (config . was_set () . unstable_features () , false) ; config . set () . unstable_features (true) ; assert_eq ! (config . unstable_features () , true) ; } # [nightly_only_test] # [test] fn test_unstable_from_toml () { let config = Config :: from_toml ("unstable_features = true" , Path :: new ("./rustfmt.toml")) . unwrap () ; assert_eq ! (config . was_set () . unstable_features () , true) ; assert_eq ! (config . unstable_features () , true) ; } # [test] fn test_set_cli () { let mut config = Config :: default () ; assert_eq ! (config . was_set () . edition () , false) ; assert_eq ! (config . was_set_cli () . edition () , false) ; config . set () . edition (Edition :: Edition2021) ; assert_eq ! (config . was_set () . edition () , false) ; assert_eq ! (config . was_set_cli () . edition () , false) ; config . set_cli () . edition (Edition :: Edition2021) ; assert_eq ! (config . was_set () . edition () , false) ; assert_eq ! (config . was_set_cli () . edition () , true) ; assert_eq ! (config . was_set_cli () . emit_mode () , false) ; } # [cfg (test)] mod deprecated_option_merge_imports { use super :: * ; # [nightly_only_test] # [test] fn test_old_option_set () { let toml = r#"
                unstable_features = true
                merge_imports = true
            "# ; let config = Config :: from_toml (toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert_eq ! (config . imports_granularity () , ImportGranularity :: Crate) ; } # [nightly_only_test] # [test] fn test_both_set () { let toml = r#"
                unstable_features = true
                merge_imports = true
                imports_granularity = "Preserve"
            "# ; let config = Config :: from_toml (toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert_eq ! (config . imports_granularity () , ImportGranularity :: Preserve) ; } # [nightly_only_test] # [test] fn test_new_overridden () { let toml = r#"
                unstable_features = true
                merge_imports = true
            "# ; let mut config = Config :: from_toml (toml , Path :: new ("./rustfmt.toml")) . unwrap () ; config . override_value ("imports_granularity" , "Preserve") ; assert_eq ! (config . imports_granularity () , ImportGranularity :: Preserve) ; } # [nightly_only_test] # [test] fn test_old_overridden () { let toml = r#"
                unstable_features = true
                imports_granularity = "Module"
            "# ; let mut config = Config :: from_toml (toml , Path :: new ("./rustfmt.toml")) . unwrap () ; config . override_value ("merge_imports" , "true") ; assert_eq ! (config . imports_granularity () , ImportGranularity :: Module) ; } } # [cfg (test)] mod use_small_heuristics { use super :: * ; # [test] fn test_default_sets_correct_widths () { let toml = r#"
                use_small_heuristics = "Default"
                max_width = 200
            "# ; let config = Config :: from_toml (toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert_eq ! (config . array_width () , 120) ; assert_eq ! (config . attr_fn_like_width () , 140) ; assert_eq ! (config . chain_width () , 120) ; assert_eq ! (config . fn_call_width () , 120) ; assert_eq ! (config . single_line_if_else_max_width () , 100) ; assert_eq ! (config . struct_lit_width () , 36) ; assert_eq ! (config . struct_variant_width () , 70) ; } # [test] fn test_max_sets_correct_widths () { let toml = r#"
                use_small_heuristics = "Max"
                max_width = 120
            "# ; let config = Config :: from_toml (toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert_eq ! (config . array_width () , 120) ; assert_eq ! (config . attr_fn_like_width () , 120) ; assert_eq ! (config . chain_width () , 120) ; assert_eq ! (config . fn_call_width () , 120) ; assert_eq ! (config . single_line_if_else_max_width () , 120) ; assert_eq ! (config . struct_lit_width () , 120) ; assert_eq ! (config . struct_variant_width () , 120) ; } # [test] fn test_off_sets_correct_widths () { let toml = r#"
                use_small_heuristics = "Off"
                max_width = 100
            "# ; let config = Config :: from_toml (toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert_eq ! (config . array_width () , usize :: MAX) ; assert_eq ! (config . attr_fn_like_width () , usize :: MAX) ; assert_eq ! (config . chain_width () , usize :: MAX) ; assert_eq ! (config . fn_call_width () , usize :: MAX) ; assert_eq ! (config . single_line_if_else_max_width () , 0) ; assert_eq ! (config . struct_lit_width () , 0) ; assert_eq ! (config . struct_variant_width () , 0) ; } # [test] fn test_override_works_with_default () { let toml = r#"
                use_small_heuristics = "Default"
                array_width = 20
                attr_fn_like_width = 40
                chain_width = 20
                fn_call_width = 90
                single_line_if_else_max_width = 40
                struct_lit_width = 30
                struct_variant_width = 34
            "# ; let config = Config :: from_toml (toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert_eq ! (config . array_width () , 20) ; assert_eq ! (config . attr_fn_like_width () , 40) ; assert_eq ! (config . chain_width () , 20) ; assert_eq ! (config . fn_call_width () , 90) ; assert_eq ! (config . single_line_if_else_max_width () , 40) ; assert_eq ! (config . struct_lit_width () , 30) ; assert_eq ! (config . struct_variant_width () , 34) ; } # [test] fn test_override_with_max () { let toml = r#"
                use_small_heuristics = "Max"
                array_width = 20
                attr_fn_like_width = 40
                chain_width = 20
                fn_call_width = 90
                single_line_if_else_max_width = 40
                struct_lit_width = 30
                struct_variant_width = 34
            "# ; let config = Config :: from_toml (toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert_eq ! (config . array_width () , 20) ; assert_eq ! (config . attr_fn_like_width () , 40) ; assert_eq ! (config . chain_width () , 20) ; assert_eq ! (config . fn_call_width () , 90) ; assert_eq ! (config . single_line_if_else_max_width () , 40) ; assert_eq ! (config . struct_lit_width () , 30) ; assert_eq ! (config . struct_variant_width () , 34) ; } # [test] fn test_override_with_off () { let toml = r#"
                use_small_heuristics = "Off"
                array_width = 20
                attr_fn_like_width = 40
                chain_width = 20
                fn_call_width = 90
                single_line_if_else_max_width = 40
                struct_lit_width = 30
                struct_variant_width = 34
            "# ; let config = Config :: from_toml (toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert_eq ! (config . array_width () , 20) ; assert_eq ! (config . attr_fn_like_width () , 40) ; assert_eq ! (config . chain_width () , 20) ; assert_eq ! (config . fn_call_width () , 90) ; assert_eq ! (config . single_line_if_else_max_width () , 40) ; assert_eq ! (config . struct_lit_width () , 30) ; assert_eq ! (config . struct_variant_width () , 34) ; } # [test] fn test_fn_call_width_config_exceeds_max_width () { let toml = r#"
                max_width = 90
                fn_call_width = 95
            "# ; let config = Config :: from_toml (toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert_eq ! (config . fn_call_width () , 90) ; } # [test] fn test_attr_fn_like_width_config_exceeds_max_width () { let toml = r#"
                max_width = 80
                attr_fn_like_width = 90
            "# ; let config = Config :: from_toml (toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert_eq ! (config . attr_fn_like_width () , 80) ; } # [test] fn test_struct_lit_config_exceeds_max_width () { let toml = r#"
                max_width = 78
                struct_lit_width = 90
            "# ; let config = Config :: from_toml (toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert_eq ! (config . struct_lit_width () , 78) ; } # [test] fn test_struct_variant_width_config_exceeds_max_width () { let toml = r#"
                max_width = 80
                struct_variant_width = 90
            "# ; let config = Config :: from_toml (toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert_eq ! (config . struct_variant_width () , 80) ; } # [test] fn test_array_width_config_exceeds_max_width () { let toml = r#"
                max_width = 60
                array_width = 80
            "# ; let config = Config :: from_toml (toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert_eq ! (config . array_width () , 60) ; } # [test] fn test_chain_width_config_exceeds_max_width () { let toml = r#"
                max_width = 80
                chain_width = 90
            "# ; let config = Config :: from_toml (toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert_eq ! (config . chain_width () , 80) ; } # [test] fn test_single_line_if_else_max_width_config_exceeds_max_width () { let toml = r#"
                max_width = 70
                single_line_if_else_max_width = 90
            "# ; let config = Config :: from_toml (toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert_eq ! (config . single_line_if_else_max_width () , 70) ; } # [test] fn test_override_fn_call_width_exceeds_max_width () { let mut config = Config :: default () ; config . override_value ("fn_call_width" , "101") ; assert_eq ! (config . fn_call_width () , 100) ; } # [test] fn test_override_attr_fn_like_width_exceeds_max_width () { let mut config = Config :: default () ; config . override_value ("attr_fn_like_width" , "101") ; assert_eq ! (config . attr_fn_like_width () , 100) ; } # [test] fn test_override_struct_lit_exceeds_max_width () { let mut config = Config :: default () ; config . override_value ("struct_lit_width" , "101") ; assert_eq ! (config . struct_lit_width () , 100) ; } # [test] fn test_override_struct_variant_width_exceeds_max_width () { let mut config = Config :: default () ; config . override_value ("struct_variant_width" , "101") ; assert_eq ! (config . struct_variant_width () , 100) ; } # [test] fn test_override_array_width_exceeds_max_width () { let mut config = Config :: default () ; config . override_value ("array_width" , "101") ; assert_eq ! (config . array_width () , 100) ; } # [test] fn test_override_chain_width_exceeds_max_width () { let mut config = Config :: default () ; config . override_value ("chain_width" , "101") ; assert_eq ! (config . chain_width () , 100) ; } # [test] fn test_override_single_line_if_else_max_width_exceeds_max_width () { let mut config = Config :: default () ; config . override_value ("single_line_if_else_max_width" , "101") ; assert_eq ! (config . single_line_if_else_max_width () , 100) ; } } # [cfg (test)] mod partially_unstable_option { use super :: mock :: { Config , PartiallyUnstableOption } ; # [doc = " From the command line, we can override with a stable variant."] # [test] fn test_override_stable_value () { let mut config = Config :: default () ; config . override_value ("partially_unstable_option" , "V2") ; assert_eq ! (config . partially_unstable_option () , PartiallyUnstableOption :: V2) ; } # [doc = " From the command line, we can override with an unstable variant."] # [test] fn test_override_unstable_value () { let mut config = Config :: default () ; config . override_value ("partially_unstable_option" , "V3") ; assert_eq ! (config . partially_unstable_option () , PartiallyUnstableOption :: V3) ; } } # [test] fn test_override_skip_macro_invocations () { let mut config = Config :: default () ; config . override_value ("skip_macro_invocations" , r#"["*", "println"]"#) ; assert_eq ! (config . skip_macro_invocations () , MacroSelectors (vec ! [MacroSelector :: All , MacroSelector :: Name (MacroName :: new ("println" . to_owned ()))])) ; } # [cfg (test)] mod required_version { use super :: * ; # [allow (dead_code)] fn get_current_version () -> semver :: Version { semver :: Version :: parse (env ! ("CARGO_PKG_VERSION")) . unwrap () } # [nightly_only_test] # [test] fn test_required_version_default () { let config = Config :: default () ; assert ! (config . version_meets_requirement ()) ; } # [nightly_only_test] # [test] fn test_current_required_version () { let toml = format ! ("required_version=\"{}\"" , env ! ("CARGO_PKG_VERSION")) ; let config = Config :: from_toml (& toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert ! (config . version_meets_requirement ()) ; } # [nightly_only_test] # [test] fn test_required_version_above () { let toml = "required_version=\"1000.0.0\"" ; let config = Config :: from_toml (toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert ! (! config . version_meets_requirement ()) ; } # [nightly_only_test] # [test] fn test_required_version_below () { let versions = vec ! ["0.0.0" , "0.0.1" , "0.1.0"] ; for version in versions { let toml = format ! ("required_version=\"{}\"" , version . to_string ()) ; let config = Config :: from_toml (& toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert ! (! config . version_meets_requirement ()) ; } } # [nightly_only_test] # [test] fn test_required_version_tilde () { let toml = format ! ("required_version=\"~{}\"" , env ! ("CARGO_PKG_VERSION")) ; let config = Config :: from_toml (& toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert ! (config . version_meets_requirement ()) ; } # [nightly_only_test] # [test] fn test_required_version_caret () { let current_version = get_current_version () ; for minor in current_version . minor .. 0 { let toml = format ! ("required_version=\"^{}.{}.0\"" , current_version . major . to_string () , minor . to_string ()) ; let config = Config :: from_toml (& toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert ! (! config . version_meets_requirement ()) ; } } # [nightly_only_test] # [test] fn test_required_version_greater_than () { let toml = "required_version=\">1.0.0\"" ; let config = Config :: from_toml (toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert ! (config . version_meets_requirement ()) ; } # [nightly_only_test] # [test] fn test_required_version_less_than () { let toml = "required_version=\"<1.0.0\"" ; let config = Config :: from_toml (toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert ! (! config . version_meets_requirement ()) ; } # [nightly_only_test] # [test] fn test_required_version_range () { let current_version = get_current_version () ; let toml = format ! ("required_version=\">={}.0.0, <{}.0.0\"" , current_version . major , current_version . major + 1) ; let config = Config :: from_toml (& toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert ! (config . version_meets_requirement ()) ; } # [nightly_only_test] # [test] fn test_required_version_exact_boundary () { let toml = format ! ("required_version=\"{}\"" , get_current_version () . to_string ()) ; let config = Config :: from_toml (& toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert ! (config . version_meets_requirement ()) ; } # [nightly_only_test] # [test] fn test_required_version_pre_release () { let toml = format ! ("required_version=\"^{}-alpha\"" , get_current_version () . to_string ()) ; let config = Config :: from_toml (& toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert ! (config . version_meets_requirement ()) ; } # [nightly_only_test] # [test] fn test_required_version_with_build_metadata () { let toml = format ! ("required_version=\"{}+build.1\"" , get_current_version () . to_string ()) ; let config = Config :: from_toml (& toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert ! (config . version_meets_requirement ()) ; } # [nightly_only_test] # [test] fn test_required_version_invalid_specification () { let toml = "required_version=\"not.a.version\"" ; let config = Config :: from_toml (toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert ! (! config . version_meets_requirement ()) } # [nightly_only_test] # [test] fn test_required_version_complex_range () { let current_version = get_current_version () ; let toml = format ! ("required_version=\">={}.0.0, <{}.0.0, ~{}.{}.0\"" , current_version . major , current_version . major + 1 , current_version . major , current_version . minor) ; let config = Config :: from_toml (& toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert ! (config . version_meets_requirement ()) ; } # [nightly_only_test] # [test] fn test_required_version_wildcard_major () { let toml = "required_version=\"1.x\"" ; let config = Config :: from_toml (toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert ! (config . version_meets_requirement ()) ; } # [nightly_only_test] # [test] fn test_required_version_wildcard_any () { let toml = "required_version=\"*\"" ; let config = Config :: from_toml (toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert ! (config . version_meets_requirement ()) ; } # [nightly_only_test] # [test] fn test_required_version_major_version_zero () { let toml = "required_version=\"0.1.0\"" ; let config = Config :: from_toml (toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert ! (! config . version_meets_requirement ()) ; } # [nightly_only_test] # [test] fn test_required_version_future_major_version () { let toml = "required_version=\"3.0.0\"" ; let config = Config :: from_toml (toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert ! (! config . version_meets_requirement ()) ; } # [nightly_only_test] # [test] fn test_required_version_fail_different_operator () { let toml = "required_version=\"!=1.0.0\"" ; let config = Config :: from_toml (toml , Path :: new ("./rustfmt.toml")) . unwrap () ; assert ! (! config . version_meets_requirement ()) ; } } # [cfg (test)] mod check_semver_version { use super :: * ; # [test] fn test_exact_version_match () { assert ! (check_semver_version ("1.0.0" , "1.0.0")) ; assert ! (! check_semver_version ("1.0.0" , "1.1.0")) ; assert ! (! check_semver_version ("1.0.0" , "1.0.1")) ; assert ! (! check_semver_version ("1.0.0" , "2.1.0")) ; assert ! (! check_semver_version ("1.0.0" , "0.1.0")) ; assert ! (! check_semver_version ("1.0.0" , "0.0.1")) ; } # [test] fn test_version_mismatch () { assert ! (! check_semver_version ("2.0.0" , "1.0.0")) ; } # [test] fn test_patch_version_greater () { assert ! (check_semver_version ("^1.0.0" , "1.0.1")) ; } # [test] fn test_minor_version_greater () { assert ! (check_semver_version ("^1.0.0" , "1.1.0")) ; } # [test] fn test_major_version_less () { assert ! (! check_semver_version ("1.0.0" , "0.9.0")) ; } # [test] fn test_prerelease_less_than_release () { assert ! (! check_semver_version ("1.0.0" , "1.0.0-alpha")) ; } # [test] fn test_prerelease_version_specific_match () { assert ! (check_semver_version ("1.0.0-alpha" , "1.0.0-alpha")) ; } # [test] fn test_build_metadata_ignored () { assert ! (check_semver_version ("1.0.0" , "1.0.0+build.1")) ; } # [test] fn test_greater_than_requirement () { assert ! (check_semver_version (">1.0.0" , "1.1.0")) ; } # [test] fn test_less_than_requirement_fails_when_greater () { assert ! (! check_semver_version ("<1.0.0" , "1.1.0")) ; } # [test] fn test_caret_requirement_matches_minor_update () { assert ! (check_semver_version ("^1.1.0" , "1.2.0")) ; } # [test] fn test_tilde_requirement_matches_patch_update () { assert ! (check_semver_version ("~1.0.0" , "1.0.1")) ; } # [test] fn test_range_requirement_inclusive () { assert ! (check_semver_version (">=1.0.0, <2.0.0" , "1.5.0")) ; } # [test] fn test_pre_release_specific_match () { assert ! (check_semver_version ("1.0.0-alpha.1" , "1.0.0-alpha.1")) ; } # [test] fn test_pre_release_non_match_when_requiring_release () { assert ! (! check_semver_version ("1.0.0" , "1.0.0-alpha.1")) ; } # [test] fn test_invalid_or () { assert ! (! check_semver_version ("1.0.0 || 2.0.0" , "1.0.0")) ; assert ! (! check_semver_version ("1.0.0 || 2.0.0" , "2.0.0")) ; assert ! (! check_semver_version ("1.0.0 || 2.0.0" , "3.0.0")) ; } # [test] fn test_wildcard_match_minor () { assert ! (check_semver_version ("1.*" , "1.1.0")) ; assert ! (check_semver_version ("1.*, <2.0.0" , "1.1.0")) ; } # [test] fn test_wildcard_mismatch () { assert ! (! check_semver_version ("1.*, <2.0.0" , "2.1.0")) ; assert ! (! check_semver_version ("1.*, <2.0.0" , "2.0.0")) ; assert ! (! check_semver_version ("1.*, <2.*" , "2.1.0")) ; assert ! (! check_semver_version ("1.*, <2.*" , "2.0.0")) ; assert ! (! check_semver_version ("1.*, >2.0.0" , "1.1.0")) ; assert ! (! check_semver_version ("1.*, >2.0.0" , "1.0.0")) ; assert ! (! check_semver_version ("1.*, >2.*" , "1.1.0")) ; assert ! (! check_semver_version ("1.*, >2.*" , "1.0.0")) ; assert ! (! check_semver_version ("<1.5.0, >1.10.*" , "1.6.0")) ; } # [test] fn test_wildcard_match_major () { assert ! (check_semver_version ("2.*" , "2.0.0")) ; } # [test] fn test_wildcard_match_patch () { assert ! (check_semver_version ("1.0.*" , "1.0.1")) ; } # [test] fn test_invalid_inputs () { assert ! (! check_semver_version ("not.a.requirement" , "1.0.0")) ; assert ! (! check_semver_version ("1.0.0" , "not.a.version")) ; } # [test] fn test_version_with_pre_release_and_build () { assert ! (check_semver_version ("1.0.0-alpha" , "1.0.0-alpha+001")) ; } # [test] fn test_pre_release_numeric_vs_alphanumeric () { assert ! (! check_semver_version ("^1.0.0-alpha.beta" , "1.0.0-alpha.1")) ; assert ! (check_semver_version ("^1.0.0-alpha.1" , "1.0.0-alpha.beta")) ; } # [test] fn test_wildcard_any () { assert ! (check_semver_version ("*" , "1.0.0")) ; assert ! (check_semver_version ("*" , "1.0.0+build")) ; } # [test] fn test_pre_release_lexicographic_ordering () { assert ! (check_semver_version ("^1.0.0-alpha.alpha" , "1.0.0-alpha.beta" ,)) ; assert ! (! check_semver_version ("^1.0.0-alpha.beta" , "1.0.0-alpha.alpha" ,)) ; } # [test] fn test_wildcard_any_with_range () { assert ! (! check_semver_version ("*, <2.0.0" , "1.0.0")) ; assert ! (! check_semver_version ("*, 1.0.0" , "1.5.0")) ; } } }
};
}
