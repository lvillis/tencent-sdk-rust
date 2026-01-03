use crate::{
    Result,
    client::RequestOptions,
    types::cdn::{UpdateDomainConfigRequest, UpdateDomainConfigResponse},
};

#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct CdnService {
    client: Client,
}

#[cfg(feature = "async")]
impl CdnService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn update_domain_config(
        &self,
        request: &UpdateDomainConfigRequest,
    ) -> Result<UpdateDomainConfigResponse> {
        self.client.execute(request, None).await
    }

    pub async fn update_domain_config_with_options(
        &self,
        request: &UpdateDomainConfigRequest,
        options: &RequestOptions,
    ) -> Result<UpdateDomainConfigResponse> {
        self.client.execute(request, Some(options)).await
    }
}

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;

#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingCdnService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingCdnService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    pub fn update_domain_config(
        &self,
        request: &UpdateDomainConfigRequest,
    ) -> Result<UpdateDomainConfigResponse> {
        self.client.execute(request, None)
    }

    pub fn update_domain_config_with_options(
        &self,
        request: &UpdateDomainConfigRequest,
        options: &RequestOptions,
    ) -> Result<UpdateDomainConfigResponse> {
        self.client.execute(request, Some(options))
    }
}
