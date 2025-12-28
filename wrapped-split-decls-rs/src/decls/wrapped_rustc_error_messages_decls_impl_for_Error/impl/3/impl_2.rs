use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Error for TranslationBundleError { fn source (& self) -> Option < & (dyn Error + 'static) > { match self { TranslationBundleError :: ReadFtl (e) => Some (e) , TranslationBundleError :: ParseFtl (e) => Some (e) , TranslationBundleError :: AddResource (e) => Some (e) , TranslationBundleError :: MissingLocale => None , TranslationBundleError :: ReadLocalesDir (e) => Some (e) , TranslationBundleError :: ReadLocalesDirEntry (e) => Some (e) , TranslationBundleError :: LocaleIsNotDir => None , } } }
}