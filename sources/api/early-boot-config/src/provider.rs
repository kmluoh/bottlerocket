//! The provider module owns the `PlatformDataProvider` trait

use crate::settings::SettingsJson;
use async_trait::async_trait;

#[cfg(bottlerocket_platform = "aws-dev")]
mod local_file;

#[cfg(any(bottlerocket_platform = "aws", bottlerocket_platform = "aws-dev"))]
mod aws;
#[cfg(any(bottlerocket_platform = "aws", bottlerocket_platform = "aws-dev"))]
pub(crate) use aws::AwsDataProvider as Platform;

#[cfg(bottlerocket_platform = "vmware")]
mod vmware;
#[cfg(bottlerocket_platform = "vmware")]
pub(crate) use vmware::VmwareDataProvider as Platform;

// FIXME: KMLUOH
#[cfg(bottlerocket_platform = "kvm-dev")]
mod local_file;
#[cfg(bottlerocket_platform = "kvm-dev")]
mod kvm;
#[cfg(bottlerocket_platform = "kvm-dev")]
pub(crate) use kvm::KvmDataProvider as Platform;

/// Support for new platforms can be added by implementing this trait.
#[async_trait]
pub(crate) trait PlatformDataProvider {
    /// You should return a list of SettingsJson, representing the settings changes you want to
    /// send to the API.
    ///
    /// This is a list so that handling multiple data sources within a platform can feel more
    /// natural; you can also send all changes in one entry if you like.
    async fn platform_data(
        &self,
    ) -> std::result::Result<Vec<SettingsJson>, Box<dyn std::error::Error>>;
}
