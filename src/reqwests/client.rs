use reqwest::Response;

use crate::prelude::*;

pub static JSON_SCHEMA: LazyLock<serde_json::Value> = LazyLock::new(|| {
    serde_json::json!({
        "name": "card",
        "strict": true,

        "schema": {
            "type": "object",

            "properties": {
                "key": {
                    "type": "string"
                },
                "ask": {
                    "type": "string"
                },
                "answer": {
                    "type": "string"
                },
                "explanation": {
                    "type": "string"
                }
            },

            "required": [
                "key",
                "ask",
                "answer",
                "explanation"
            ],

            "additionalProperties": false
        }
    })
});

#[derive(Deserialize, Serialize)]
pub struct WrapResponse {
    pub content: Card,
    pub tokens_used: u64,
    pub tokens_left: u64,
    pub code: String,
}

pub struct ApiConnector<'a> {
    pub cl: &'a Client,
    pub api: &'a ApiSettings,
}

impl<'a> ApiConnector<'a> {
    pub fn new(cl: &'a Client, api: &'a ApiSettings) -> Self {
        Self { cl, api }
    }
}

pub async fn req<'a>(
    conn: &ApiConnector<'a>,
    json_struct_send: impl for<'c> Deserialize<'c> + Serialize,
) -> RResult<Response> {
    Ok(conn
        .cl
        .post(&conn.api.url)
        .bearer_auth(&conn.api.token)
        .json(&json_struct_send)
        .send()
        .await?)
}

pub struct Promts<'a> {
    pub promt_system: &'a str,
    pub promt_type_answer: &'a str,
    pub promt: &'a str,
}

impl<'a> Promts<'a> {
    pub fn new(promt_system: &'a str, promt_type_answer: &'a str, promt: &'a str) -> Self {
        Self {
            promt_system,
            promt_type_answer,
            promt,
        }
    }
}

pub trait ClientJsonSchema {
    fn build(&self, model: &str, promts: Promts, schema: &Value) -> Value;
}

pub trait ClientCardResp: ClientJsonSchema {
    fn req<'a>(
        &'a self,
        conn: &'a ApiConnector<'a>,
        promts: Promts<'a>,
    ) -> Pin<Box<dyn Future<Output = RResult<WrapResponse>> + 'a>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude_tests::prelude::*;

    #[tokio::test]
    async fn req_res_1() {
        let api = &CONFIG.api.api["groq1"];
        let resp = req(
            &ApiConnector::new(&CLIENT, &api),
            GroqClient.build(
                &api.model,
                Promts::new(ALL, RT_VARIANT, PROMT_TEST),
                &JSON_SCHEMA,
            ),
        )
        .await
        .unwrap();
        let status = resp.status().to_string();
        assert_eq_pr!(status, "200 OK");
    }
}
