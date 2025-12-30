// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
impl Widget for & StackContainer { fn render (self , area : Rect , buf : & mut Buffer) { let layout = Layout :: default () . direction (self . direction) . constraints (self . widgets . iter () . map (| (_ , constraint) | * constraint)) . split (area) ; let widgets = self . widgets . iter () . map (| (widget , _) | widget) ; for (widget , area) in zip (widgets , layout . iter ()) { widget . render_ref (* area , buf) ; } } }
};
}
