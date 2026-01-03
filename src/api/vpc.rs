use crate::{
    Result,
    client::RequestOptions,
    types::vpc::{
        CreateSubnetRequest, CreateSubnetResponse, CreateVpcRequest, CreateVpcResponse,
        DescribeSubnetsRequest, DescribeSubnetsResponse, DescribeVpcsRequest, DescribeVpcsResponse,
    },
};

#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct VpcService {
    client: Client,
}

#[cfg(feature = "async")]
impl VpcService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn describe_vpcs(
        &self,
        request: &DescribeVpcsRequest,
    ) -> Result<DescribeVpcsResponse> {
        self.client.execute(request, None).await
    }

    pub async fn describe_vpcs_with_options(
        &self,
        request: &DescribeVpcsRequest,
        options: &RequestOptions,
    ) -> Result<DescribeVpcsResponse> {
        self.client.execute(request, Some(options)).await
    }

    pub async fn create_vpc(&self, request: &CreateVpcRequest) -> Result<CreateVpcResponse> {
        self.client.execute(request, None).await
    }

    pub async fn create_vpc_with_options(
        &self,
        request: &CreateVpcRequest,
        options: &RequestOptions,
    ) -> Result<CreateVpcResponse> {
        self.client.execute(request, Some(options)).await
    }

    pub async fn create_subnet(
        &self,
        request: &CreateSubnetRequest,
    ) -> Result<CreateSubnetResponse> {
        self.client.execute(request, None).await
    }

    pub async fn create_subnet_with_options(
        &self,
        request: &CreateSubnetRequest,
        options: &RequestOptions,
    ) -> Result<CreateSubnetResponse> {
        self.client.execute(request, Some(options)).await
    }

    pub async fn describe_subnets(
        &self,
        request: &DescribeSubnetsRequest,
    ) -> Result<DescribeSubnetsResponse> {
        self.client.execute(request, None).await
    }

    pub async fn describe_subnets_with_options(
        &self,
        request: &DescribeSubnetsRequest,
        options: &RequestOptions,
    ) -> Result<DescribeSubnetsResponse> {
        self.client.execute(request, Some(options)).await
    }
}

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;

#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingVpcService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingVpcService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    pub fn describe_vpcs(&self, request: &DescribeVpcsRequest) -> Result<DescribeVpcsResponse> {
        self.client.execute(request, None)
    }

    pub fn describe_vpcs_with_options(
        &self,
        request: &DescribeVpcsRequest,
        options: &RequestOptions,
    ) -> Result<DescribeVpcsResponse> {
        self.client.execute(request, Some(options))
    }

    pub fn create_vpc(&self, request: &CreateVpcRequest) -> Result<CreateVpcResponse> {
        self.client.execute(request, None)
    }

    pub fn create_vpc_with_options(
        &self,
        request: &CreateVpcRequest,
        options: &RequestOptions,
    ) -> Result<CreateVpcResponse> {
        self.client.execute(request, Some(options))
    }

    pub fn create_subnet(&self, request: &CreateSubnetRequest) -> Result<CreateSubnetResponse> {
        self.client.execute(request, None)
    }

    pub fn create_subnet_with_options(
        &self,
        request: &CreateSubnetRequest,
        options: &RequestOptions,
    ) -> Result<CreateSubnetResponse> {
        self.client.execute(request, Some(options))
    }

    pub fn describe_subnets(
        &self,
        request: &DescribeSubnetsRequest,
    ) -> Result<DescribeSubnetsResponse> {
        self.client.execute(request, None)
    }

    pub fn describe_subnets_with_options(
        &self,
        request: &DescribeSubnetsRequest,
        options: &RequestOptions,
    ) -> Result<DescribeSubnetsResponse> {
        self.client.execute(request, Some(options))
    }
}
