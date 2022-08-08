use std::sync::Arc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::connect::Connect + Clone + Send + Sync> {
  configuration: Arc<Configuration<C>>,
}

impl<C: hyper::client::connect::Connect + Clone + Send + Sync> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Arc::new(configuration);

    APIClient {
      configuration: rc.clone(),
    }
  }


}
