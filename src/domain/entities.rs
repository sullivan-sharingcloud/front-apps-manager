
pub struct MicroFrontend {
    pub name: String,
    pub version: Option<Version>,
}

pub struct Version {
    pub number: String
}

impl MicroFrontend {
    pub fn new(name: &str) -> MicroFrontend{
        MicroFrontend {
            name: name.to_string(),
            version: None
        }
    }
}