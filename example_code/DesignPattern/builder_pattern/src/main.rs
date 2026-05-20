//! 构建者模式 (Builder Pattern) 示例：构建 HTTP 请求对象
//!
//! 这个模块做什么：
//! - 使用 Builder（方法链 + `build()`）来构建字段较多、部分字段可选的对象。
//!
//! 场景说明：
//! - 模拟一个 HTTP 请求对象：包含 url、method、headers、body、timeout、retry 等字段。
//! - 调用方通过 `RequestBuilder` 按需设置字段，最后 `build()` 得到 `Request`。

use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

/// 一个简化的 HTTP 请求对象（最终产物 Product）。
#[derive(Debug, Clone)]
struct Request {
    url: String,
    method: HttpMethod,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
    timeout_ms: u64,
    retry: u8,
}

/// HTTP 请求构建器（Builder）。
#[derive(Debug, Clone)]
struct RequestBuilder {
    url: Option<String>,
    method: HttpMethod,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
    timeout_ms: u64,
    retry: u8,
}

#[derive(Debug)]
enum BuildError {
    MissingUrl,
}

/// 输出请求的关键信息，帮助理解“构建后的对象”会被如何使用。
fn print_request_summary(req: &Request) {
    let body_len = req.body.as_ref().map(|b| b.len()).unwrap_or(0);
    println!(
        "summary: method={:?} url={} headers={} body_len={} timeout_ms={} retry={}",
        req.method,
        req.url,
        req.headers.len(),
        body_len,
        req.timeout_ms,
        req.retry
    );
}

impl RequestBuilder {
    /// 创建一个新的构建器，带有默认值。
    fn new() -> Self {
        RequestBuilder {
            url: None,
            method: HttpMethod::Get,
            headers: HashMap::new(),
            body: None,
            timeout_ms: 3_000,
            retry: 0,
        }
    }

    /// 设置请求 URL（必填字段）。
    fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }

    /// 设置 HTTP 方法。
    fn method(mut self, method: HttpMethod) -> Self {
        self.method = method;
        self
    }

    /// 追加一个请求头。
    fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    /// 设置请求体。
    fn body_bytes(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    /// 设置超时时间（毫秒）。
    fn timeout_ms(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }

    /// 设置重试次数。
    fn retry(mut self, retry: u8) -> Self {
        self.retry = retry;
        self
    }

    /// 构建最终的 `Request`。
    ///
    /// # Errors
    /// - 当必填字段未设置时返回错误（如 URL 未设置）。
    fn build(self) -> Result<Request, BuildError> {
        let url = self.url.ok_or(BuildError::MissingUrl)?;
        Ok(Request {
            url,
            method: self.method,
            headers: self.headers,
            body: self.body,
            timeout_ms: self.timeout_ms,
            retry: self.retry,
        })
    }
}

fn main() {
    let create_user = RequestBuilder::new()
        .url("https://example.com/api/v1/users")
        .method(HttpMethod::Post)
        .header("Content-Type", "application/json")
        .header("X-Request-Id", "req-123")
        .timeout_ms(5_000)
        .retry(2)
        .body_bytes(br#"{"name":"alice"}"#.to_vec())
        .build()
        .unwrap();

    let update_user = RequestBuilder::new()
        .url("https://example.com/api/v1/users/42")
        .method(HttpMethod::Put)
        .header("Content-Type", "application/json")
        .timeout_ms(4_000)
        .retry(1)
        .body_bytes(br#"{"name":"alice_v2"}"#.to_vec())
        .build()
        .unwrap();

    let delete_user = RequestBuilder::new()
        .url("https://example.com/api/v1/users/42")
        .method(HttpMethod::Delete)
        .header("X-Request-Id", "req-999")
        .timeout_ms(2_000)
        .build()
        .unwrap();

    println!("create_user = {create_user:#?}");
    print_request_summary(&create_user);
    println!();
    println!("update_user = {update_user:#?}");
    print_request_summary(&update_user);
    println!();
    println!("delete_user = {delete_user:#?}");
    print_request_summary(&delete_user);
}
