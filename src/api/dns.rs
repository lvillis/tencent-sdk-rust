use crate::{
    Result,
    client::RequestOptions,
    types::dns::{
        CreateTxtRecordRequest, CreateTxtRecordResponse, DeleteRecordRequest, DeleteRecordResponse,
        ModifyTxtRecordRequest, ModifyTxtRecordResponse,
    },
};

#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct DnsService {
    client: Client,
}

#[cfg(feature = "async")]
impl DnsService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create_txt_record(
        &self,
        request: &CreateTxtRecordRequest,
    ) -> Result<CreateTxtRecordResponse> {
        self.client.execute(request, None).await
    }

    pub async fn create_txt_record_with_options(
        &self,
        request: &CreateTxtRecordRequest,
        options: &RequestOptions,
    ) -> Result<CreateTxtRecordResponse> {
        self.client.execute(request, Some(options)).await
    }

    pub async fn modify_txt_record(
        &self,
        request: &ModifyTxtRecordRequest,
    ) -> Result<ModifyTxtRecordResponse> {
        self.client.execute(request, None).await
    }

    pub async fn modify_txt_record_with_options(
        &self,
        request: &ModifyTxtRecordRequest,
        options: &RequestOptions,
    ) -> Result<ModifyTxtRecordResponse> {
        self.client.execute(request, Some(options)).await
    }

    pub async fn delete_record(
        &self,
        request: &DeleteRecordRequest,
    ) -> Result<DeleteRecordResponse> {
        self.client.execute(request, None).await
    }

    pub async fn delete_record_with_options(
        &self,
        request: &DeleteRecordRequest,
        options: &RequestOptions,
    ) -> Result<DeleteRecordResponse> {
        self.client.execute(request, Some(options)).await
    }
}

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;

#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingDnsService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingDnsService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    pub fn create_txt_record(
        &self,
        request: &CreateTxtRecordRequest,
    ) -> Result<CreateTxtRecordResponse> {
        self.client.execute(request, None)
    }

    pub fn create_txt_record_with_options(
        &self,
        request: &CreateTxtRecordRequest,
        options: &RequestOptions,
    ) -> Result<CreateTxtRecordResponse> {
        self.client.execute(request, Some(options))
    }

    pub fn modify_txt_record(
        &self,
        request: &ModifyTxtRecordRequest,
    ) -> Result<ModifyTxtRecordResponse> {
        self.client.execute(request, None)
    }

    pub fn modify_txt_record_with_options(
        &self,
        request: &ModifyTxtRecordRequest,
        options: &RequestOptions,
    ) -> Result<ModifyTxtRecordResponse> {
        self.client.execute(request, Some(options))
    }

    pub fn delete_record(&self, request: &DeleteRecordRequest) -> Result<DeleteRecordResponse> {
        self.client.execute(request, None)
    }

    pub fn delete_record_with_options(
        &self,
        request: &DeleteRecordRequest,
        options: &RequestOptions,
    ) -> Result<DeleteRecordResponse> {
        self.client.execute(request, Some(options))
    }
}
