#[cfg(feature = "ssl")]
use openssl::ssl::SslConnector;
use std::time::Duration;

use authenticators::Authenticator;

pub struct ClusterSslConfig<'a, A: Authenticator + Sized>(pub Vec<NodeSslConfig<'a, A>>);

#[derive(Clone)]
pub struct NodeSslConfig<'a, A> {
  pub addr: &'a str,
  pub authenticator: A,
  pub ssl_connector: SslConnector,
  pub max_size: u32,
  pub min_idle: Option<u32>,
  pub max_lifetime: Option<Duration>,
  pub idle_timeout: Option<Duration>,
  pub connection_timeout: Duration,
}

pub struct NodeSslConfigBuilder<'a, A> {
  addr: &'a str,
  authenticator: A,
  ssl_connector: SslConnector,
  max_size: Option<u32>,
  min_idle: Option<u32>,
  max_lifetime: Option<Duration>,
  idle_timeout: Option<Duration>,
  connection_timeout: Option<Duration>,
}

impl<'a, A: Authenticator + Sized> NodeSslConfigBuilder<'a, A> {
  const DEFAULT_MAX_SIZE: u32 = 10;
  const DEFAULT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);

  pub fn new<'b>(
    addr: &'b str,
    authenticator: A,
    ssl_connector: SslConnector,
  ) -> NodeSslConfigBuilder<'b, A> {
    NodeSslConfigBuilder {
      addr,
      authenticator,
      ssl_connector,
      max_size: None,
      min_idle: None,
      max_lifetime: None,
      idle_timeout: None,
      connection_timeout: None,
    }
  }

  pub fn max_size(mut self, size: u32) -> Self {
    self.max_size = Some(size);
    self
  }

  pub fn min_idle(mut self, min_idle: Option<u32>) -> Self {
    self.max_size = min_idle;
    self
  }

  pub fn max_lifetime(mut self, max_lifetime: Option<Duration>) -> Self {
    self.max_lifetime = max_lifetime;
    self
  }

  pub fn idle_timeout(mut self, idle_timeout: Option<Duration>) -> Self {
    self.idle_timeout = idle_timeout;
    self
  }

  pub fn connection_timeout(mut self, connection_timeout: Duration) -> Self {
    self.connection_timeout = Some(connection_timeout);
    self
  }

  pub fn authenticator(mut self, authenticator: A) -> Self {
    self.authenticator = authenticator;
    self
  }

  pub fn build(self) -> NodeSslConfig<'a, A> {
    NodeSslConfig {
      addr: self.addr,
      authenticator: self.authenticator,
      ssl_connector: self.ssl_connector,

      max_size: self.max_size.unwrap_or(Self::DEFAULT_MAX_SIZE),
      min_idle: self.min_idle,
      max_lifetime: self.max_lifetime,
      idle_timeout: self.idle_timeout,
      connection_timeout: self
        .connection_timeout
        .unwrap_or(Self::DEFAULT_CONNECTION_TIMEOUT),
    }
  }
}
