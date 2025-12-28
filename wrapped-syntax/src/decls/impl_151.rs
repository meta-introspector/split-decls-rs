macro_rules! deps {
    () => {
        SyntaxElement!();
        PositionRepr!();
        SyntaxNode!();
        ChangeKind!();
        Change!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl Change { fn target_range (& self) -> TextRange { match self { Change :: Insert (target , _) | Change :: InsertAll (target , _) => match & target . repr { PositionRepr :: FirstChild (parent) => TextRange :: at (parent . first_child_or_token () . unwrap () . text_range () . start () , 0 . into () ,) , PositionRepr :: After (child) => TextRange :: at (child . text_range () . end () , 0 . into ()) , } , Change :: Replace (target , _) | Change :: ReplaceWithMany (target , _) => target . text_range () , Change :: ReplaceAll (range , _) => { range . start () . text_range () . cover (range . end () . text_range ()) } } } fn target_parent (& self) -> SyntaxNode { match self { Change :: Insert (target , _) | Change :: InsertAll (target , _) => target . parent () , Change :: Replace (target , _) | Change :: ReplaceWithMany (target , _) => match target { SyntaxElement :: Node (target) => target . parent () . unwrap_or_else (| | target . clone ()) , SyntaxElement :: Token (target) => target . parent () . unwrap () , } , Change :: ReplaceAll (target , _) => target . start () . parent () . unwrap () , } } fn change_kind (& self) -> ChangeKind { match self { Change :: Insert (_ , _) | Change :: InsertAll (_ , _) => ChangeKind :: Insert , Change :: Replace (_ , _) | Change :: ReplaceWithMany (_ , _) => ChangeKind :: Replace , Change :: ReplaceAll (_ , _) => ChangeKind :: ReplaceRange , } } }
    };
}

impl_151!()