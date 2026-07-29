# Reference
<details><summary><code>client.<a href="/src/client.rs">moderate_a_record</a>(request: ModerateRequest) -> Result&lt;ModerateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create or update a record and return its moderation result immediately.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use insion::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = InsionClient::new(config).expect("Failed to build client");
    client
        .moderate_a_record(
            &ModerateRequest {
                client_id: "clientId".to_string(),
                name: "name".to_string(),
                entity: "entity".to_string(),
                content: Content::String("content".to_string()),
                client_url: None,
                metadata: None,
                user: None,
                passthrough: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**passthrough:** `Option<bool>` — Moderate without persisting the record's name or content, or the user's email, name, or username.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">ingest_a_record</a>(request: IngestRecordRequest) -> Result&lt;IngestRecordResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create or update a content record for asynchronous moderation. Results are delivered through webhook events when moderation is performed.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use insion::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = InsionClient::new(config).expect("Failed to build client");
    client
        .ingest_a_record(
            &IngestRecordRequest(RecordInput {
                client_id: "clientId".to_string(),
                client_url: None,
                name: "name".to_string(),
                entity: "entity".to_string(),
                content: Content::String("content".to_string()),
                metadata: None,
                user: None,
            }),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">delete_a_record</a>(request: DeleteApiV1IngestRequest) -> Result&lt;SuccessResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Remove a record from the moderation system by its client ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use insion::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = InsionClient::new(config).expect("Failed to build client");
    client
        .delete_a_record(
            &DeleteAPIV1IngestRequest {
                client_id: "clientId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**client_id:** `String` — Your unique identifier for the record.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">ingest_a_user</a>(request: UserInput) -> Result&lt;IngestUserResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create or update a user without ingesting a record.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use insion::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = InsionClient::new(config).expect("Failed to build client");
    client
        .ingest_a_user(
            &UserInput {
                client_id: "clientId".to_string(),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">list_records</a>(limit: Option&lt;Option&lt;i64&gt;&gt;, starting_after: Option&lt;Option&lt;String&gt;&gt;, ending_before: Option&lt;Option&lt;String&gt;&gt;, client_id: Option&lt;Option&lt;String&gt;&gt;, user: Option&lt;Option&lt;String&gt;&gt;, entity: Option&lt;Option&lt;String&gt;&gt;, status: Option&lt;Option&lt;GetApiV1RecordsRequestStatus&gt;&gt;) -> Result&lt;ListRecordsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

List the records belonging to the authenticated organization.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use insion::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = InsionClient::new(config).expect("Failed to build client");
    client
        .list_records(
            &ListRecordsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<i64>` — Maximum number of items to return.
    
</dd>
</dl>

<dl>
<dd>

**starting_after:** `Option<String>` — Return items after this Insion ID. Cannot be used with ending_before.
    
</dd>
</dl>

<dl>
<dd>

**ending_before:** `Option<String>` — Return items before this Insion ID. Cannot be used with starting_after.
    
</dd>
</dl>

<dl>
<dd>

**client_id:** `Option<String>` — Filter by your record identifier.
    
</dd>
</dl>

<dl>
<dd>

**user:** `Option<String>` — Filter by Insion user ID.
    
</dd>
</dl>

<dl>
<dd>

**entity:** `Option<String>` — Filter by record entity.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<GetApiV1RecordsRequestStatus>` — Filter by moderation status.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">retrieve_a_record</a>(record_id: String) -> Result&lt;RecordResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve one record by its Insion record ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use insion::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = InsionClient::new(config).expect("Failed to build client");
    client
        .retrieve_a_record(&"recordId".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**record_id:** `String` — Insion record ID.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">list_users</a>(limit: Option&lt;Option&lt;i64&gt;&gt;, starting_after: Option&lt;Option&lt;String&gt;&gt;, ending_before: Option&lt;Option&lt;String&gt;&gt;, client_id: Option&lt;Option&lt;String&gt;&gt;, email: Option&lt;Option&lt;String&gt;&gt;, status: Option&lt;Option&lt;GetApiV1UsersRequestStatus&gt;&gt;, user: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListUsersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

List the users belonging to the authenticated organization.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use insion::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = InsionClient::new(config).expect("Failed to build client");
    client
        .list_users(
            &ListUsersQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<i64>` — Maximum number of items to return.
    
</dd>
</dl>

<dl>
<dd>

**starting_after:** `Option<String>` — Return items after this Insion ID. Cannot be used with ending_before.
    
</dd>
</dl>

<dl>
<dd>

**ending_before:** `Option<String>` — Return items before this Insion ID. Cannot be used with starting_after.
    
</dd>
</dl>

<dl>
<dd>

**client_id:** `Option<String>` — Filter by your user identifier.
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<String>` — Filter by user email.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<GetApiV1UsersRequestStatus>` — Filter by user action status.
    
</dd>
</dl>

<dl>
<dd>

**user:** `Option<String>` — Filter by Insion user ID.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">retrieve_a_user</a>(user_id: String) -> Result&lt;UserResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve one user by its Insion user ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use insion::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = InsionClient::new(config).expect("Failed to build client");
    client.retrieve_a_user(&"userId".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**user_id:** `String` — Insion user ID.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">create_an_appeal</a>(user_id: String, request: PostApiV1UsersUserIdCreateAppealRequest) -> Result&lt;CreateAppealResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create an appeal for a suspended user. Appeals must be enabled for the organization.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use insion::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = InsionClient::new(config).expect("Failed to build client");
    client
        .create_an_appeal(
            &"userId".to_string(),
            &PostAPIV1UsersUserIDCreateAppealRequest {
                text: "text".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**user_id:** `String` — Insion user ID.
    
</dd>
</dl>

<dl>
<dd>

**text:** `String` — The appeal message.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

