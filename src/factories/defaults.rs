use clickhouse::Client;

use crate::repositories::clickhouse::ClickhouseStorage;

use super::{
    clickhouse::ClickhouseClientConfig,
    generator::{DefaultTransferGenerator, TransferGenConfig, TransferGenerator},
};

pub fn generator() -> GeneratorBuilder {
    GeneratorBuilder {
        config: TransferGenConfig::default(),
    }
}

pub struct GeneratorBuilder {
    config: TransferGenConfig,
}

impl GeneratorBuilder {
    pub fn with_config(self, config: TransferGenConfig) -> Self {
        Self { config }
    }

    pub fn build(self) -> Box<dyn TransferGenerator> {
        Box::new(DefaultTransferGenerator {
            config: self.config,
        })
    }
}
