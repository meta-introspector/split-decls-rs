// Generated macro for impl_75 (impl)
macro_rules! Depcrateimpl_75 {
() => {
// Module: crate
// Provides: {"impl_75"}
// Dependencies: {}
impl ChangedFile { # [doc = " Returns `true` if the change is not [`Delete`](ChangeKind::Delete)."] pub fn exists (& self) -> bool { ! matches ! (self . change , Change :: Delete) } # [doc = " Returns `true` if the change is [`Create`](ChangeKind::Create) or"] # [doc = " [`Delete`](Change::Delete)."] pub fn is_created_or_deleted (& self) -> bool { matches ! (self . change , Change :: Create (_ , _) | Change :: Delete) } # [doc = " Returns `true` if the change is [`Create`](ChangeKind::Create)."] pub fn is_created (& self) -> bool { matches ! (self . change , Change :: Create (_ , _)) } # [doc = " Returns `true` if the change is [`Modify`](ChangeKind::Modify)."] pub fn is_modified (& self) -> bool { matches ! (self . change , Change :: Modify (_ , _)) } pub fn kind (& self) -> ChangeKind { match self . change { Change :: Create (_ , _) => ChangeKind :: Create , Change :: Modify (_ , _) => ChangeKind :: Modify , Change :: Delete => ChangeKind :: Delete , } } }
};
}
