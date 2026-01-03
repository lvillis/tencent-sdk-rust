use crate::{
    Result,
    client::RequestOptions,
    types::cvm::{
        DescribeImagesRequest, DescribeImagesResponse, DescribeInstanceVncUrlRequest,
        DescribeInstanceVncUrlResponse, DescribeInstancesRequest, DescribeInstancesResponse,
        GenericActionResponse, ModifyInstancesProjectRequest, RebootInstancesRequest,
        ResetInstancesPasswordRequest, RunInstancesRequest, RunInstancesResponse,
        StartInstancesRequest, StopInstancesRequest, TerminateInstancesRequest,
    },
};

#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct CvmService {
    client: Client,
}

#[cfg(feature = "async")]
impl CvmService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn describe_instances(
        &self,
        request: &DescribeInstancesRequest,
    ) -> Result<DescribeInstancesResponse> {
        self.client.execute(request, None).await
    }

    pub async fn describe_instances_with_options(
        &self,
        request: &DescribeInstancesRequest,
        options: &RequestOptions,
    ) -> Result<DescribeInstancesResponse> {
        self.client.execute(request, Some(options)).await
    }

    pub async fn reset_instances_password(
        &self,
        request: &ResetInstancesPasswordRequest,
    ) -> Result<GenericActionResponse> {
        self.client.execute(request, None).await
    }

    pub async fn reset_instances_password_with_options(
        &self,
        request: &ResetInstancesPasswordRequest,
        options: &RequestOptions,
    ) -> Result<GenericActionResponse> {
        self.client.execute(request, Some(options)).await
    }

    pub async fn describe_instance_vnc_url(
        &self,
        request: &DescribeInstanceVncUrlRequest,
    ) -> Result<DescribeInstanceVncUrlResponse> {
        self.client.execute(request, None).await
    }

    pub async fn describe_instance_vnc_url_with_options(
        &self,
        request: &DescribeInstanceVncUrlRequest,
        options: &RequestOptions,
    ) -> Result<DescribeInstanceVncUrlResponse> {
        self.client.execute(request, Some(options)).await
    }

    pub async fn run_instances(
        &self,
        request: &RunInstancesRequest,
    ) -> Result<RunInstancesResponse> {
        self.client.execute(request, None).await
    }

    pub async fn run_instances_with_options(
        &self,
        request: &RunInstancesRequest,
        options: &RequestOptions,
    ) -> Result<RunInstancesResponse> {
        self.client.execute(request, Some(options)).await
    }

    pub async fn start_instances(
        &self,
        request: &StartInstancesRequest,
    ) -> Result<GenericActionResponse> {
        self.client.execute(request, None).await
    }

    pub async fn start_instances_with_options(
        &self,
        request: &StartInstancesRequest,
        options: &RequestOptions,
    ) -> Result<GenericActionResponse> {
        self.client.execute(request, Some(options)).await
    }

    pub async fn reboot_instances(
        &self,
        request: &RebootInstancesRequest,
    ) -> Result<GenericActionResponse> {
        self.client.execute(request, None).await
    }

    pub async fn reboot_instances_with_options(
        &self,
        request: &RebootInstancesRequest,
        options: &RequestOptions,
    ) -> Result<GenericActionResponse> {
        self.client.execute(request, Some(options)).await
    }

    pub async fn stop_instances(
        &self,
        request: &StopInstancesRequest,
    ) -> Result<GenericActionResponse> {
        self.client.execute(request, None).await
    }

    pub async fn stop_instances_with_options(
        &self,
        request: &StopInstancesRequest,
        options: &RequestOptions,
    ) -> Result<GenericActionResponse> {
        self.client.execute(request, Some(options)).await
    }

    pub async fn modify_instances_project(
        &self,
        request: &ModifyInstancesProjectRequest,
    ) -> Result<GenericActionResponse> {
        self.client.execute(request, None).await
    }

    pub async fn modify_instances_project_with_options(
        &self,
        request: &ModifyInstancesProjectRequest,
        options: &RequestOptions,
    ) -> Result<GenericActionResponse> {
        self.client.execute(request, Some(options)).await
    }

    pub async fn terminate_instances(
        &self,
        request: &TerminateInstancesRequest,
    ) -> Result<GenericActionResponse> {
        self.client.execute(request, None).await
    }

    pub async fn terminate_instances_with_options(
        &self,
        request: &TerminateInstancesRequest,
        options: &RequestOptions,
    ) -> Result<GenericActionResponse> {
        self.client.execute(request, Some(options)).await
    }

    pub async fn describe_images(
        &self,
        request: &DescribeImagesRequest,
    ) -> Result<DescribeImagesResponse> {
        self.client.execute(request, None).await
    }

    pub async fn describe_images_with_options(
        &self,
        request: &DescribeImagesRequest,
        options: &RequestOptions,
    ) -> Result<DescribeImagesResponse> {
        self.client.execute(request, Some(options)).await
    }
}

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;

#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingCvmService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingCvmService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    pub fn describe_instances(
        &self,
        request: &DescribeInstancesRequest,
    ) -> Result<DescribeInstancesResponse> {
        self.client.execute(request, None)
    }

    pub fn describe_instances_with_options(
        &self,
        request: &DescribeInstancesRequest,
        options: &RequestOptions,
    ) -> Result<DescribeInstancesResponse> {
        self.client.execute(request, Some(options))
    }

    pub fn reset_instances_password(
        &self,
        request: &ResetInstancesPasswordRequest,
    ) -> Result<GenericActionResponse> {
        self.client.execute(request, None)
    }

    pub fn reset_instances_password_with_options(
        &self,
        request: &ResetInstancesPasswordRequest,
        options: &RequestOptions,
    ) -> Result<GenericActionResponse> {
        self.client.execute(request, Some(options))
    }

    pub fn describe_instance_vnc_url(
        &self,
        request: &DescribeInstanceVncUrlRequest,
    ) -> Result<DescribeInstanceVncUrlResponse> {
        self.client.execute(request, None)
    }

    pub fn describe_instance_vnc_url_with_options(
        &self,
        request: &DescribeInstanceVncUrlRequest,
        options: &RequestOptions,
    ) -> Result<DescribeInstanceVncUrlResponse> {
        self.client.execute(request, Some(options))
    }

    pub fn run_instances(&self, request: &RunInstancesRequest) -> Result<RunInstancesResponse> {
        self.client.execute(request, None)
    }

    pub fn run_instances_with_options(
        &self,
        request: &RunInstancesRequest,
        options: &RequestOptions,
    ) -> Result<RunInstancesResponse> {
        self.client.execute(request, Some(options))
    }

    pub fn start_instances(
        &self,
        request: &StartInstancesRequest,
    ) -> Result<GenericActionResponse> {
        self.client.execute(request, None)
    }

    pub fn start_instances_with_options(
        &self,
        request: &StartInstancesRequest,
        options: &RequestOptions,
    ) -> Result<GenericActionResponse> {
        self.client.execute(request, Some(options))
    }

    pub fn reboot_instances(
        &self,
        request: &RebootInstancesRequest,
    ) -> Result<GenericActionResponse> {
        self.client.execute(request, None)
    }

    pub fn reboot_instances_with_options(
        &self,
        request: &RebootInstancesRequest,
        options: &RequestOptions,
    ) -> Result<GenericActionResponse> {
        self.client.execute(request, Some(options))
    }

    pub fn stop_instances(&self, request: &StopInstancesRequest) -> Result<GenericActionResponse> {
        self.client.execute(request, None)
    }

    pub fn stop_instances_with_options(
        &self,
        request: &StopInstancesRequest,
        options: &RequestOptions,
    ) -> Result<GenericActionResponse> {
        self.client.execute(request, Some(options))
    }

    pub fn modify_instances_project(
        &self,
        request: &ModifyInstancesProjectRequest,
    ) -> Result<GenericActionResponse> {
        self.client.execute(request, None)
    }

    pub fn modify_instances_project_with_options(
        &self,
        request: &ModifyInstancesProjectRequest,
        options: &RequestOptions,
    ) -> Result<GenericActionResponse> {
        self.client.execute(request, Some(options))
    }

    pub fn terminate_instances(
        &self,
        request: &TerminateInstancesRequest,
    ) -> Result<GenericActionResponse> {
        self.client.execute(request, None)
    }

    pub fn terminate_instances_with_options(
        &self,
        request: &TerminateInstancesRequest,
        options: &RequestOptions,
    ) -> Result<GenericActionResponse> {
        self.client.execute(request, Some(options))
    }

    pub fn describe_images(
        &self,
        request: &DescribeImagesRequest,
    ) -> Result<DescribeImagesResponse> {
        self.client.execute(request, None)
    }

    pub fn describe_images_with_options(
        &self,
        request: &DescribeImagesRequest,
        options: &RequestOptions,
    ) -> Result<DescribeImagesResponse> {
        self.client.execute(request, Some(options))
    }
}
