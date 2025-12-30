// Generated macro for slugify (function)
macro_rules! Depcrateslugify {
() => {
// Module: crate
// Provides: {"slugify"}
// Dependencies: {}
# [doc = " Convert any unicode string to an ascii \"slug\" (useful for file names/url components)"] # [doc = ""] # [doc = " The returned \"slug\" will consist of a-z, 0-9, and '-'. Furthermore, a slug will"] # [doc = " never contain more than one '-' in a row and will never start or end with '-'."] # [doc = ""] # [doc = " ```rust"] # [doc = " use self::slug::slugify;"] # [doc = ""] # [doc = " assert_eq!(slugify(\"My Test String!!!1!1\"), \"my-test-string-1-1\");"] # [doc = " assert_eq!(slugify(\"test\\nit   now!\"), \"test-it-now\");"] # [doc = " assert_eq!(slugify(\"  --test_-_cool\"), \"test-cool\");"] # [doc = " assert_eq!(slugify(\"Æúű--cool?\"), \"aeuu-cool\");"] # [doc = " assert_eq!(slugify(\"You & Me\"), \"you-me\");"] # [doc = " assert_eq!(slugify(\"user@example.com\"), \"user-example-com\");"] # [doc = " ```"] pub fn slugify < S : AsRef < str > > (s : S) -> String { _slugify (s . as_ref ()) }
};
}
