//! Create and manage `bevy_diagnostics`

use bevy::{
    diagnostic::{Diagnostic, DiagnosticId, Diagnostics},
    prelude::{App, Plugin, ResMut},
};

pub struct MemoryCpuDiagnosticsPlugin;

impl Plugin for MemoryCpuDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::setup_diagnostic_systems)
            .add_system(Self::diagnostic_measurements);
    }
}

impl MemoryCpuDiagnosticsPlugin {
    pub const STATIC_MEMORY_USE: DiagnosticId =
        DiagnosticId::from_u128(242230241449459897678106279349291549618);

    pub const DYNAMIC_MEMORY_USE: DiagnosticId =
        DiagnosticId::from_u128(260842125471598085650860941644900914766);

    pub const CPU_USE: DiagnosticId =
        DiagnosticId::from_u128(309468735612075730284885959000788573045);

    /// Add diagnostics to the project
    ///
    /// All diagnostics will need to be added to a system that runs before diagnostic logging happens.
    pub fn setup_diagnostic_systems(mut diagnostics: ResMut<Diagnostics>) {
        // Add a static memory use diagnostic
        diagnostics.add(
            Diagnostic::new(Self::STATIC_MEMORY_USE, "static_memory_use", 10).with_suffix("%"),
        );
        // Add a dynamic memory use diagnostic
        diagnostics.add(
            Diagnostic::new(Self::DYNAMIC_MEMORY_USE, "dynamic_memory_use", 10).with_suffix("%"),
        );
        // Add a cpu use diagnostic
        diagnostics.add(Diagnostic::new(Self::CPU_USE, "cpu_use", 10).with_suffix("%"));
    }

    //FIX: add logic to actually measure things
    /// Take active measurements of diagnostic data.
    pub fn diagnostic_measurements(mut diagnostics: ResMut<Diagnostics>) {
        diagnostics.add_measurement(Self::STATIC_MEMORY_USE, || 0.1);
        diagnostics.add_measurement(Self::DYNAMIC_MEMORY_USE, || 0.1);
        diagnostics.add_measurement(Self::CPU_USE, || 0.1)
    }
}
