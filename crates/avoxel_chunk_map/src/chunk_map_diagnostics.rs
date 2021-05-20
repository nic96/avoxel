use bevy::diagnostic::{Diagnostic, DiagnosticId, Diagnostics};
use bevy::prelude::*;

pub const GEN_TIMES: DiagnosticId =
    DiagnosticId::from_u128(188146834822086093741974488528456902483);
pub const MESH_TIMES: DiagnosticId =
    DiagnosticId::from_u128(188146834822086093741974488528456902484);
pub const CHUNK_COMPRESSION: DiagnosticId =
    DiagnosticId::from_u128(188146834822086093741974488528456902485);
pub const COMPRESSION_TIMES: DiagnosticId =
    DiagnosticId::from_u128(188146834822086093741974488528456902486);

pub fn setup_diagnostics(mut diagnostics: ResMut<Diagnostics>) {
    diagnostics.add(Diagnostic::new(GEN_TIMES, "chunk_gen_times", 20));
    diagnostics.add(Diagnostic::new(MESH_TIMES, "chunk_mesh_times", 20));
    diagnostics.add(Diagnostic::new(
        CHUNK_COMPRESSION,
        "chunk_compression_ratio",
        20,
    ));
    diagnostics.add(Diagnostic::new(
        COMPRESSION_TIMES,
        "chunk_compression_times",
        20,
    ));
}
