use std::net::Ipv4Addr;

use serde::{Deserialize, Serialize};

use crate::error::{EngineError, EngineErrorCause, EngineErrorScope};
use crate::models::Context;

pub mod cloudflare;

#[derive(Serialize, Deserialize, Clone)]
pub enum Kind {
    CLOUDFLARE,
}

pub trait DnsProvider {
    fn context(&self) -> &Context;
    fn kind(&self) -> Kind;
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn name_with_id(&self) -> String {
        format!("{} ({})", self.name(), self.id())
    }
    fn account(&self) -> &str;
    fn token(&self) -> &str;
    fn domain(&self) -> &str;
    fn resolvers(&self) -> Vec<Ipv4Addr>;
    fn is_valid(&self) -> Result<(), EngineError>;
    fn engine_error_scope(&self) -> EngineErrorScope {
        EngineErrorScope::DnsProvider(self.id().to_string(), self.name().to_string())
    }
    fn engine_error(&self, cause: EngineErrorCause, message: String) -> EngineError {
        EngineError::new(
            cause,
            self.engine_error_scope(),
            self.context().execution_id(),
            Some(message),
        )
    }
}
