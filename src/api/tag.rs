use crate::{
    Result,
    client::RequestOptions,
    types::tag::{DescribeProjectsRequest, DescribeProjectsResponse},
};

#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct TagService {
    client: Client,
}

#[cfg(feature = "async")]
impl TagService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn describe_projects(
        &self,
        request: &DescribeProjectsRequest,
    ) -> Result<DescribeProjectsResponse> {
        self.client.execute(request, None).await
    }

    pub async fn describe_projects_with_options(
        &self,
        request: &DescribeProjectsRequest,
        options: &RequestOptions,
    ) -> Result<DescribeProjectsResponse> {
        self.client.execute(request, Some(options)).await
    }
}

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;

#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingTagService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingTagService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    pub fn describe_projects(
        &self,
        request: &DescribeProjectsRequest,
    ) -> Result<DescribeProjectsResponse> {
        self.client.execute(request, None)
    }

    pub fn describe_projects_with_options(
        &self,
        request: &DescribeProjectsRequest,
        options: &RequestOptions,
    ) -> Result<DescribeProjectsResponse> {
        self.client.execute(request, Some(options))
    }
}
