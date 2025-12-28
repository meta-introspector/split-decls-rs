macro_rules! is_combining_mark {
    () => {
        # [doc = " Return whether the given character is a combining mark (`General_Category=Mark`)"] pub fn is_combining_mark (c : char) -> bool { mph_lookup (c . into () , COMBINING_MARK_SALT , COMBINING_MARK_KV , bool_lookup_fk , bool_lookup_fv , false ,) }
    };
}

is_combining_mark!();