use serde_json::Value;

use crate::v1::api::Client;
use crate::v1::error::APIError;
use crate::v1::resources::fine_tune::{
    CreateFineTuneParameters, DeletedFineTuneModel, FineTune, FineTuneEvent,
};

#[deprecated(since = "0.2.12")]
pub struct FineTunes<'a> {
    pub client: &'a Client,
}

impl Client {
    #[allow(deprecated)]
    #[deprecated(since = "0.2.12")]
    pub fn fine_tunes(&self) -> FineTunes {
        FineTunes { client: self }
    }
}

#[allow(deprecated)]
impl FineTunes<'_> {
    #[deprecated(since = "0.2.12")]
    pub async fn create(&self, parameters: CreateFineTuneParameters) -> Result<FineTune, APIError> {
        let response = self.client.post("/fine-tunes", &parameters).await?;

        let value: Value = serde_json::from_str(&response).unwrap();
        let fine_tune_response: FineTune = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(fine_tune_response)
    }

    #[deprecated(since = "0.2.12")]
    pub async fn list(&self) -> Result<Vec<FineTune>, APIError> {
        let response = self.client.get("/fine-tunes").await?;

        let value: Value = serde_json::from_str(&response).unwrap();
        let fine_tunes: Vec<FineTune> = serde_json::from_value(value["data"].clone())
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(fine_tunes)
    }

    #[deprecated(since = "0.2.12")]
    pub async fn retrieve(&self, id: &str) -> Result<FineTune, APIError> {
        let response = self
            .client
            .get(format!("/fine-tunes/{id}").as_str())
            .await?;

        let value: Value = serde_json::from_str(&response).unwrap();
        let fine_tune_response: FineTune = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(fine_tune_response)
    }

    #[deprecated(since = "0.2.12")]
    pub async fn cancel(&self, id: &str) -> Result<FineTune, APIError> {
        let response = self
            .client
            .post(format!("/fine-tunes/{id}/cancel").as_str(), &())
            .await?;

        let value: Value = serde_json::from_str(&response).unwrap();
        let fine_tune_response: FineTune = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(fine_tune_response)
    }

    #[deprecated(since = "0.2.12")]
    pub async fn list_events(&self, id: &str) -> Result<Vec<FineTuneEvent>, APIError> {
        let response = self
            .client
            .get(format!("/fine-tunes/{id}/events").as_str())
            .await?;

        let value: Value = serde_json::from_str(&response).unwrap();
        let fine_tune_response: Vec<FineTuneEvent> = serde_json::from_value(value["data"].clone())
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(fine_tune_response)
    }

    #[deprecated(since = "0.2.12")]
    pub async fn delete_fine_tune_model(&self, id: &str) -> Result<DeletedFineTuneModel, APIError> {
        let response = self.client.delete(format!("/models/{id}").as_str()).await?;

        let value: Value = serde_json::from_str(&response).unwrap();
        let delete_model_response: DeletedFineTuneModel =
            serde_json::from_value(value["data"].clone())
                .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(delete_model_response)
    }
}
