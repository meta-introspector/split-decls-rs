// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl Html { # [doc = " Parse an HTML document from a string."] # [doc = ""] # [doc = " This is just a thin wrapper around `scraper::Html::parse_document` to"] # [doc = " keep the exported API surface simpler."] pub fn parse (source : & str) -> Html { Html { inner : scraper :: Html :: parse_document (source) , } } # [doc = " Get the first item in the document matching a string selector. Returns"] # [doc = " Some()"] # [doc = ""] # [doc = " If the selector is not a valid CSS selector, panics rather than"] # [doc = " returning a [`Result`] for convenience."] pub fn select_first < 'a > (& 'a self , selector : & 'a str ,) -> Option < scraper :: ElementRef < 'a > > { let selector = scraper :: Selector :: parse (selector) . unwrap () ; self . inner . select (& selector) . nth (0) } }
};
}
