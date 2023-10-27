use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;

#[no_mangle]
pub fn translate(
    text: &str,
    from: &str,
    to: &str,
    _detect: &str,
    _needs: HashMap<String, String>,
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::blocking::ClientBuilder::new().build()?;

    let res: Value = client
        .post(format!(
            "http://res.d.hjfile.cn/v10/dict/translation/{from}/{to}"
        ))
        .header("Host", "res.d.hjfile.cn")
        .header("Origin", "http://res.d.hjfile.cn")
        .header("Referer", "http://res.d.hjfile.cn/app/trans")
        .header(
            "User-Agent",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36",
        )
        .header(
            "Content-Type",
            "application/x-www-form-urlencoded; charset=UTF-8",
        )
        .header(
            "Cookie",
            "HJ_UID=390f25c7-c9f3-b237-f639-62bd23cd431f; HJC_USRC=uzhi; HJC_NUID=1",
        )
        .form(&json!({"content":text}))
        .send()?
        .json()?;

    fn parse_result(res: &Value) -> Option<String> {
        let result = res
            .as_object()?
            .get("data")?
            .as_object()?
            .get("content")?
            .as_str()?;

        Some(result.to_string())
    }
    if let Some(result) = parse_result(&res) {
        return Ok(Value::String(result));
    } else {
        return Err(format!("Response Parse Error: {}", &res.to_string()).into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let needs = HashMap::new();
        let result = translate("你好 世界！", "auto", "en", "zh_cn", needs).unwrap();
        println!("{result}");
    }
}
