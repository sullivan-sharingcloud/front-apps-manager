use super::super::entities::MicroFrontend;

pub fn list_micro_frontends() -> Vec<MicroFrontend> {
    vec!(MicroFrontend::new("reception-desk"))
}