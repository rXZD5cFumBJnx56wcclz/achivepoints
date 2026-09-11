use serde_json::Value;

use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct GroqResp {
    id: String,
    object: String,
    created: u128,
    model: String,
    choices: Vec<GroqRespChoice>,
    usage: GroqRespUsage,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GroqRespChoice {
    pub index: usize,
    pub message: GroqRespChoiceMsg,
    pub finish_reason: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GroqRespChoiceMsg {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GroqRespUsage {
    pub queue_time: f64,
    pub prompt_tokens: u64,
    pub prompt_time: f64,
    pub completion_tokens: u64,
    pub completion_time: f64,
    pub total_tokens: u64,
    pub total_time: f64,
}

pub struct GroqClient;

impl ClientJsonSchema for GroqClient {
    fn build(&self, model: &str, promts: Promts, schema: &Value) -> Value {
        serde_json::json!({
            "model": model,
            "messages": [
                {
                    "role": "system",
                    "content": promts.promt_system.replace("{}", promts.promt_type_answer),
                },
                {
                    "role": "user",
                    "content": promts.promt
                }
            ],

            "response_format": {
                "type": "json_schema",
                "json_schema": schema,
            }
        })
    }
}

impl ClientCardResp for GroqClient {
    fn req<'a>(
        &'a self,
        conn: &'a ApiConnector<'a>,
        promts: Promts<'a>,
    ) -> Pin<Box<dyn Future<Output = RResult<WrapResponse>> + 'a>> {
        Box::pin(async move {
            let resp = req(conn, self.build(&conn.api.model, promts, &JSON_SCHEMA)).await?;
            let code = resp.status().to_string();
            let strct = resp.json::<GroqResp>().await?;
            Ok(WrapResponse {
                tokens_left: 0,
                content: serde_json5::from_str(&strct.choices[0].message.content)?,
                tokens_used: strct.usage.completion_tokens,
                code: code,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude_tests::prelude::*;

    #[tokio::test]
    async fn req_res_1() {
        let v = GroqClient
            .req(
                &ApiConnector::new(&CLIENT, CONFIG.api.api.get("groq1").unwrap()),
                Promts::new(ALL, RT_VARIANT, PROMT_TEST),
            )
            .await
            .unwrap()
            .content;
        assert!(!v.key.is_empty());
        assert!(!v.ask.is_empty());
        assert!(!v.answer.is_empty());
    }
}
