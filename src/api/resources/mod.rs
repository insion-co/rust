//! Service clients and API endpoints
//!
//! This module provides the client implementations for all available services.

use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct InsionClient {
    pub config: ClientConfig,
    pub http_client: HttpClient,
}

impl InsionClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Create or update a record and return its moderation result immediately.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn moderate_a_record(
        &self,
        request: &ModerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<ModerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v1/moderate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Create or update a content record for asynchronous moderation. Results are delivered through webhook events when moderation is performed.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn ingest_a_record(
        &self,
        request: &IngestRecordRequest,
        options: Option<RequestOptions>,
    ) -> Result<IngestRecordResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v1/ingest",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Remove a record from the moderation system by its client ID.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete_a_record(
        &self,
        request: &DeleteApiV1IngestRequest,
        options: Option<RequestOptions>,
    ) -> Result<SuccessResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                "api/v1/ingest",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Create or update a user without ingesting a record.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn ingest_a_user(
        &self,
        request: &UserInput,
        options: Option<RequestOptions>,
    ) -> Result<IngestUserResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v1/ingest/user",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the records belonging to the authenticated organization.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of items to return.
    /// * `starting_after` - Return items after this Insion ID. Cannot be used with ending_before.
    /// * `ending_before` - Return items before this Insion ID. Cannot be used with starting_after.
    /// * `client_id` - Filter by your record identifier.
    /// * `user` - Filter by Insion user ID.
    /// * `entity` - Filter by record entity.
    /// * `status` - Filter by moderation status.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_records(
        &self,
        request: &ListRecordsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListRecordsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "api/v1/records",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .string("starting_after", request.starting_after.clone())
                    .string("ending_before", request.ending_before.clone())
                    .string("clientId", request.client_id.clone())
                    .string("user", request.user.clone())
                    .string("entity", request.entity.clone())
                    .serialize("status", request.status.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve one record by its Insion record ID.
    ///
    /// # Arguments
    ///
    /// * `record_id` - Insion record ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn retrieve_a_record(
        &self,
        record_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RecordResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v1/records/{}", record_id),
                None,
                None,
                options,
            )
            .await
    }

    /// List the users belonging to the authenticated organization.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of items to return.
    /// * `starting_after` - Return items after this Insion ID. Cannot be used with ending_before.
    /// * `ending_before` - Return items before this Insion ID. Cannot be used with starting_after.
    /// * `client_id` - Filter by your user identifier.
    /// * `email` - Filter by user email.
    /// * `status` - Filter by user action status.
    /// * `user` - Filter by Insion user ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_users(
        &self,
        request: &ListUsersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListUsersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "api/v1/users",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .string("starting_after", request.starting_after.clone())
                    .string("ending_before", request.ending_before.clone())
                    .string("clientId", request.client_id.clone())
                    .string("email", request.email.clone())
                    .serialize("status", request.status.clone())
                    .string("user", request.user.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve one user by its Insion user ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - Insion user ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn retrieve_a_user(
        &self,
        user_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<UserResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v1/users/{}", user_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Create an appeal for a suspended user. Appeals must be enabled for the organization.
    ///
    /// # Arguments
    ///
    /// * `user_id` - Insion user ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_an_appeal(
        &self,
        user_id: &str,
        request: &PostApiV1UsersUserIdCreateAppealRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateAppealResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v1/users/{}/create_appeal", user_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
