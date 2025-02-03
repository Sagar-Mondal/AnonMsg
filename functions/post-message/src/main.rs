use aws_lambda_events::{
    encodings::Body,
    event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse},
};
use http::header::HeaderMap;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::env;

use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};

use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_utc_timestamps()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let func = service_fn(post_message_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn post_message_handler(
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    let Ok(uri) = env::var("MONGODB_URI") else {
        return Ok(ApiGatewayProxyResponse {
            status_code: 500,
            headers: HeaderMap::new(),
            multi_value_headers: HeaderMap::new(),
            body: Some(Body::Text("Database connection error".to_string())),
            is_base64_encoded: false,
        });
    };

    log::info!("Connecting to MongoDB at {}", uri);

    let Ok(client) = Client::with_uri_str(uri).await else {
        return Ok(ApiGatewayProxyResponse {
            status_code: 500,
            headers: HeaderMap::new(),
            multi_value_headers: HeaderMap::new(),
            body: Some(Body::Text("Database connection error".to_string())),
            is_base64_encoded: false,
        });
    };

    let database = client.database("anonmsg");
    let messages: Collection<Document> = database.collection("messages");

    let Some(text) = event.payload.body else {
        return Ok(ApiGatewayProxyResponse {
            status_code: 400,
            headers: HeaderMap::new(),
            multi_value_headers: HeaderMap::new(),
            body: Some(Body::Text("No message provided".to_string())),
            is_base64_encoded: false,
        });
    };

    messages
        .insert_one(doc! {
            "text": text,
            "timestamp": Utc::now().timestamp()
        })
        .await?;

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text("Successfully posted message!".to_string())),
        is_base64_encoded: false,
    })
}
