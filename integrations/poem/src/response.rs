use std::str::FromStr;

use http::{HeaderName, HeaderValue};
use poem::{web::Json, IntoResponse, Response, http::{header}};

/// Response for `async_graphql::Request`.
pub struct GraphQLResponse(pub async_graphql::Response);

impl From<async_graphql::Response> for GraphQLResponse {
    fn from(resp: async_graphql::Response) -> Self {
        Self(resp)
    }
}

impl IntoResponse for GraphQLResponse {
    fn into_response(self) -> Response {
        GraphQLBatchResponse(self.0.into()).into_response()
    }
}

/// Response for `async_graphql::BatchRequest`.
pub struct GraphQLBatchResponse(pub async_graphql::BatchResponse);

impl From<async_graphql::BatchResponse> for GraphQLBatchResponse {
    fn from(resp: async_graphql::BatchResponse) -> Self {
        Self(resp)
    }
}

impl IntoResponse for GraphQLBatchResponse {
    fn into_response(self) -> Response {
        println!("processing response");
        let mut resp = if self.0.has_raw_data() {
            println!("response has raw data");
            let raw_data = self.0.get_raw_data().unwrap();
            Response::builder()
            .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
            .body(raw_data)
        }else {
            Json(&self.0).into_response()
        }; 

        if self.0.is_ok() {
            if let Some(cache_control) = self.0.cache_control().value() {
                if let Ok(value) = cache_control.try_into() {
                    resp.headers_mut().insert("cache-control", value);
                }
            }
        }

        resp.headers_mut()
            .extend(self.0.http_headers().iter().filter_map(|(name, value)| {
                HeaderName::from_str(name.as_str())
                    .ok()
                    .zip(HeaderValue::from_bytes(value.as_bytes()).ok())
            }));
        resp
    }
}
