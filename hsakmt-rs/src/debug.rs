#[derive(Debug, PartialEq)]
pub enum DebugMode {
    All,
    None,
    Topology,
    Fmm,
}

// TODO configure rust log
pub const CURRENT: DebugMode = DebugMode::None;

pub fn debug_topology_println() -> bool {
    CURRENT == DebugMode::All || CURRENT == DebugMode::Topology
}

pub fn debug_fmm_println() -> bool {
    CURRENT == DebugMode::All || CURRENT == DebugMode::Fmm
}
