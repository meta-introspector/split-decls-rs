macro_rules! deps {
    () => {
        DebuggerVisualizerCollector!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < 'ast > rustc_ast :: visit :: Visitor < 'ast > for DebuggerVisualizerCollector < '_ > { fn visit_attribute (& mut self , attr : & 'ast Attribute) { self . check_for_debugger_visualizer (attr) ; rustc_ast :: visit :: walk_attribute (self , attr) ; } }
    };
}

impl_56!();