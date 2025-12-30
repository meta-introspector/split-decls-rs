// Generated macro for Component (struct)
macro_rules! Depcrate_common_componentComponent {
() => {
// Module: crate::common::component
// Provides: {"Component"}
// Dependencies: {}
# [doc = " Getting a component temperature information."] # [doc = ""] # [doc = " ```no_run"] # [doc = " use sysinfo::Components;"] # [doc = ""] # [doc = " let components = Components::new_with_refreshed_list();"] # [doc = " for component in &components {"] # [doc = "     if let Some(temperature) = component.temperature() {"] # [doc = "         println!(\"{} {temperature}°C\", component.label());"] # [doc = "     } else {"] # [doc = "         println!(\"{} (unknown temperature)\", component.label());"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] pub struct Component { pub (crate) inner : ComponentInner , }
};
}
