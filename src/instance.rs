use std::path::Path;

use mongodb::{bson::Document, Collection};

use crate::MasterConfig;

#[derive(Clone)]
pub struct ServiceInstance {
    pub config: MasterConfig,
    pub services: Collection<Document>,
}

impl ServiceInstance {
    pub fn load(config: &Path) -> Self {
        let config = MasterConfig::read(config);
        let services = config.mongodb.load();
        ServiceInstance { config, services }
    }
}
