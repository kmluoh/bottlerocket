use super::{PlatformDataProvider, SettingsJson};
use async_trait::async_trait;
use serde_json::json;
use snafu::{OptionExt, ResultExt};
use std::fs;
use std::path::Path;
use crate::provider::local_file::{local_file_user_data, USER_DATA_FILE};

pub(crate) struct KvmDataProvider;

#[async_trait]
impl PlatformDataProvider for KvmDataProvider {
    async fn platform_data(
        &self,
    ) -> std::result::Result<Vec<SettingsJson>, Box<dyn std::error::Error>> {
        let mut output = Vec::new();

        match local_file_user_data()? {
            None => warn!("No user data found via local file: {}", USER_DATA_FILE),
            Some(s) => output.push(s),
        }

        Ok(output)
    }
}

mod error {
    use snafu::Snafu;
    use std::io;
    use std::path::PathBuf;

    #[derive(Debug, Snafu)]
    #[snafu(visibility = "pub(super)")]
    pub(crate) enum Error {
        #[snafu(display("Unable to serialize settings from {}: {}", from, source))]
        SettingsToJSON {
            from: String,
            source: crate::settings::Error,
        },

        #[snafu(display(
            "Wrong type while deserializing, expected '{}' to be type '{}'",
            field_name,
            expected_type
        ))]
        WrongType {
            field_name: &'static str,
            expected_type: &'static str,
        },
    }
}

type Result<T> = std::result::Result<T, error::Error>;
