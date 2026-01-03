//! Shared request/response models.

pub mod billing;
pub mod cdn;
pub mod common;
pub mod cvm;
pub mod dns;
mod newtypes;
pub mod ssl;
pub mod tag;
pub mod vpc;

pub use common::{Filter, Tag};
pub use newtypes::{
    CertificateId, DomainName, ImageId, InstanceId, Region, RequestId, SecurityGroupId, SubnetId,
    VpcId,
};
