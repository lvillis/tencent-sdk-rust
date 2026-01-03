use crate::{
    Result,
    client::RequestOptions,
    types::ssl::{
        ApplyCertificateRequest, ApplyCertificateResponse, DescribeCertificateRequest,
        DescribeCertificateResponse, DownloadCertificateRequest, DownloadCertificateResponse,
        UploadCertificateRequest, UploadCertificateResponse,
    },
};

#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct SslService {
    client: Client,
}

#[cfg(feature = "async")]
impl SslService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn apply_certificate(
        &self,
        request: &ApplyCertificateRequest,
    ) -> Result<ApplyCertificateResponse> {
        self.client.execute(request, None).await
    }

    pub async fn apply_certificate_with_options(
        &self,
        request: &ApplyCertificateRequest,
        options: &RequestOptions,
    ) -> Result<ApplyCertificateResponse> {
        self.client.execute(request, Some(options)).await
    }

    pub async fn describe_certificate(
        &self,
        request: &DescribeCertificateRequest,
    ) -> Result<DescribeCertificateResponse> {
        self.client.execute(request, None).await
    }

    pub async fn describe_certificate_with_options(
        &self,
        request: &DescribeCertificateRequest,
        options: &RequestOptions,
    ) -> Result<DescribeCertificateResponse> {
        self.client.execute(request, Some(options)).await
    }

    pub async fn download_certificate(
        &self,
        request: &DownloadCertificateRequest,
    ) -> Result<DownloadCertificateResponse> {
        self.client.execute(request, None).await
    }

    pub async fn download_certificate_with_options(
        &self,
        request: &DownloadCertificateRequest,
        options: &RequestOptions,
    ) -> Result<DownloadCertificateResponse> {
        self.client.execute(request, Some(options)).await
    }

    pub async fn upload_certificate(
        &self,
        request: &UploadCertificateRequest,
    ) -> Result<UploadCertificateResponse> {
        self.client.execute(request, None).await
    }

    pub async fn upload_certificate_with_options(
        &self,
        request: &UploadCertificateRequest,
        options: &RequestOptions,
    ) -> Result<UploadCertificateResponse> {
        self.client.execute(request, Some(options)).await
    }
}

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;

#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingSslService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingSslService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    pub fn apply_certificate(
        &self,
        request: &ApplyCertificateRequest,
    ) -> Result<ApplyCertificateResponse> {
        self.client.execute(request, None)
    }

    pub fn apply_certificate_with_options(
        &self,
        request: &ApplyCertificateRequest,
        options: &RequestOptions,
    ) -> Result<ApplyCertificateResponse> {
        self.client.execute(request, Some(options))
    }

    pub fn describe_certificate(
        &self,
        request: &DescribeCertificateRequest,
    ) -> Result<DescribeCertificateResponse> {
        self.client.execute(request, None)
    }

    pub fn describe_certificate_with_options(
        &self,
        request: &DescribeCertificateRequest,
        options: &RequestOptions,
    ) -> Result<DescribeCertificateResponse> {
        self.client.execute(request, Some(options))
    }

    pub fn download_certificate(
        &self,
        request: &DownloadCertificateRequest,
    ) -> Result<DownloadCertificateResponse> {
        self.client.execute(request, None)
    }

    pub fn download_certificate_with_options(
        &self,
        request: &DownloadCertificateRequest,
        options: &RequestOptions,
    ) -> Result<DownloadCertificateResponse> {
        self.client.execute(request, Some(options))
    }

    pub fn upload_certificate(
        &self,
        request: &UploadCertificateRequest,
    ) -> Result<UploadCertificateResponse> {
        self.client.execute(request, None)
    }

    pub fn upload_certificate_with_options(
        &self,
        request: &UploadCertificateRequest,
        options: &RequestOptions,
    ) -> Result<UploadCertificateResponse> {
        self.client.execute(request, Some(options))
    }
}
