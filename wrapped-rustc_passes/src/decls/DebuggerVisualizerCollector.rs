macro_rules! DebuggerVisualizerCollector {
    () => {
        struct DebuggerVisualizerCollector < 'a > { sess : & 'a Session , visualizers : Vec < DebuggerVisualizerFile > , }
    };
}

DebuggerVisualizerCollector!();