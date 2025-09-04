use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheckResult {
    pub(crate) status: String,
    pub(crate) message: String,
    pub(crate) version: String,
}