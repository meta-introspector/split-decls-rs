use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < hir :: MacroKind > for SymbolKind { fn from (it : hir :: MacroKind) -> Self { match it { hir :: MacroKind :: Declarative | hir :: MacroKind :: DeclarativeBuiltIn => SymbolKind :: Macro , hir :: MacroKind :: ProcMacro => SymbolKind :: ProcMacro , hir :: MacroKind :: Derive | hir :: MacroKind :: DeriveBuiltIn => SymbolKind :: Derive , hir :: MacroKind :: Attr | hir :: MacroKind :: AttrBuiltIn => SymbolKind :: Attribute , } } }