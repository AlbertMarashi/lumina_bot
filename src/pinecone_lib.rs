use std::collections::HashMap;

use reqwest::{Method, Url};
use serde::{Serialize, Deserialize};
use serde_json::Value;

pub struct PineconeClient {
    pub key: String,
    pub host: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VectorQueryResponse {
    pub matches: Vec<VectorQueryMatch>,
    pub namespace: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VectorQueryMatch {
    pub id: String,
    pub score: f64,
    pub values: Vec<f64>,
    pub metadata: Value
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VectorUpsert {
    pub id: String,
    pub values: Vec<f64>,
    pub metadata: Value,
}

impl VectorUpsert {
    pub fn new(id: String, values: Vec<f64>, metadata: Value) -> Self {
        VectorUpsert {
            id,
            values,
            metadata,
        }
    }
}

impl PineconeClient {
    pub fn new() -> Self {
        PineconeClient {
            key: dotenv::var("PINECONE_KEY").unwrap(),
            host: dotenv::var("PINECONE_HOST").unwrap(),
        }
    }

    pub async fn vector_query(&self, vector: Vec<f64>, top_k: u64) -> Result<VectorQueryResponse, anyhow::Error> {
        #[derive(Serialize, Deserialize)]
        struct VectorQuery {
            vector: Vec<f64>,
            #[serde(rename = "topK")]
            top_k: u64,
            namespace: Option<String>,
            #[serde(rename = "includeMetadata")]
            include_metadata: bool,
            #[serde(rename = "includeValues")]
            include_values: bool,
        }

        let query = VectorQuery {
            vector,
            top_k,
            namespace: None,
            include_metadata: true,
            include_values: false,
        };

        let value = self.req(
            Method::POST,
            "/query",
            Some(serde_json::to_value(query)?)
        ).await?;

        Ok(serde_json::from_value(value)?)
    }

    pub async fn vector_upsert(&self, vectors: Vec<VectorUpsert>) -> Result<u64, anyhow::Error> {
        #[derive(Debug, Serialize, Deserialize)]
        struct UpsertRequest {
            vectors: Vec<VectorUpsert>,
        }

        #[derive(Serialize, Deserialize)]
        struct VectorUpsertResponse {
            #[serde(rename = "upsertedCount")]
            upserted_count: u64,
        }

        let req = UpsertRequest {
            vectors,
        };

        let value = self.req(
            Method::POST,
            "/vectors/upsert",
            Some(serde_json::to_value(req)?)
        ).await?;

        let res = serde_json::from_value::<VectorUpsertResponse>(value)?;

        Ok(res.upserted_count)
    }

    pub async fn req(&self, method: Method, path: &str, json: Option<Value>) -> Result<Value, anyhow::Error> {
        let mut req = reqwest::Request::new(method, Url::parse(&format!("https://{}{}", self.host, path))?);
        let headers = req.headers_mut();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Api-Key", self.key.parse().unwrap());
        headers.insert("Accept", "application/json".parse().unwrap());

        match json {
            Some(json) => *req.body_mut() = Some(reqwest::Body::from(json.to_string())),
            None => {}
        };

        let res = reqwest::Client::new().execute(req).await?;
        let body = res.json().await?;

        Ok(body)
    }

    pub async fn delete_all(&self) -> Result<(), anyhow::Error> {
        let _ = self.req(
            Method::POST,
            "/vectors/delete",
            Some(serde_json::json!({
                "deleteAll": true,
            }))
        ).await?;

        Ok(())
    }
}

#[tokio::test]
async fn test_vector_query() {
    let client = PineconeClient::new();

    let res = client.vector_query(
        // we want 1536 dimensions
        vec![0.0; 1536],
        10,
    ).await;
    println!("{:?}", res);
}

#[tokio::test]
async fn test_vector_upsert() {
    let client = PineconeClient::new();

    let res = client.vector_upsert(
        // we want 1536 dimensions
        vec![
            VectorUpsert::new("test1".into(), vec![0.0; 1536], serde_json::json!({})),
            VectorUpsert::new("test2".into(), vec![0.0; 1536], serde_json::json!({})),
        ]
    ).await;
    println!("{:?}", res);
}

#[tokio::test]
async fn test_delete_all() {
    let client = PineconeClient::new();

    let res = client.delete_all().await;
    println!("{:?}", res);
}