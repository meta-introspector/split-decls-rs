// Generated macro for HEADER (static)
macro_rules! Depcrate_case_mappingHEADER {
() => {
// Module: crate::case_mapping
// Provides: {"HEADER"}
// Dependencies: {}
static HEADER : & str = r"
pub fn to_lower(c: char) -> [char; 3] {
    if c.is_ascii() {
        [(c as u8).to_ascii_lowercase() as char, '\0', '\0']
    } else {
        LOWERCASE_TABLE
            .binary_search_by(|&(key, _)| key.cmp(&c))
            .map(|i| {
                let u = LOWERCASE_TABLE[i].1;
                char::from_u32(u).map(|c| [c, '\0', '\0']).unwrap_or_else(|| {
                    // SAFETY: Index comes from statically generated table
                    unsafe { *LOWERCASE_TABLE_MULTI.get_unchecked((u & (INDEX_MASK - 1)) as usize) }
                })
            })
            .unwrap_or([c, '\0', '\0'])
    }
}

pub fn to_upper(c: char) -> [char; 3] {
    if c.is_ascii() {
        [(c as u8).to_ascii_uppercase() as char, '\0', '\0']
    } else {
        UPPERCASE_TABLE
            .binary_search_by(|&(key, _)| key.cmp(&c))
            .map(|i| {
                let u = UPPERCASE_TABLE[i].1;
                char::from_u32(u).map(|c| [c, '\0', '\0']).unwrap_or_else(|| {
                    // SAFETY: Index comes from statically generated table
                    unsafe { *UPPERCASE_TABLE_MULTI.get_unchecked((u & (INDEX_MASK - 1)) as usize) }
                })
            })
            .unwrap_or([c, '\0', '\0'])
    }
}
" ;
};
}
