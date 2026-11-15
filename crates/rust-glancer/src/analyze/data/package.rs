use rg_project::{PackageResidency, Project, ProjectStats};
use serde::Serialize;

use crate::report::ReportFieldsBuilder;

#[derive(Debug, Serialize)]
pub(crate) struct PackageReport {
    pub(crate) total_count: usize,
    pub(crate) workspace_count: usize,
    pub(crate) residency_policy: String,
    pub(crate) resident_count: usize,
    pub(crate) offloaded_count: usize,
    pub(crate) unindexed_count: usize,
}

impl PackageReport {
    pub(crate) fn capture(project: &Project, stats: &ProjectStats) -> Self {
        let residency = project.package_residency_plan();
        let resident_count = residency
            .packages()
            .iter()
            .filter(|decision| matches!(decision, PackageResidency::Resident))
            .count();
        let unindexed_count = project.unindexed_package_count();
        // Unindexed packages hold empty resident stubs whatever the plan says, so they never
        // reach an artifact on disk.
        let offloaded_count = residency
            .packages()
            .len()
            .saturating_sub(resident_count + unindexed_count);

        Self {
            total_count: stats.package_count,
            workspace_count: stats.workspace_package_count,
            residency_policy: residency.policy().config_name().to_string(),
            resident_count,
            offloaded_count,
            unindexed_count,
        }
    }

    pub(super) fn append_fields(&self, fields: &mut ReportFieldsBuilder) {
        fields
            .count_as("total_count", "total", self.total_count)
            .count_as("workspace_count", "workspace", self.workspace_count)
            .text("residency_policy", &self.residency_policy)
            .count_as("resident_count", "resident", self.resident_count)
            .count_as("offloaded_count", "offloaded", self.offloaded_count)
            .count_as("unindexed_count", "unindexed", self.unindexed_count);
    }
}
