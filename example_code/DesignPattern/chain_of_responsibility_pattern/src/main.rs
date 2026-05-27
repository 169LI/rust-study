//! 责任链模式 (Chain of Responsibility Pattern) 示例：Web 请求中间件链。
//!
//! 这个模块做什么：
//! - 把请求处理拆成多个独立处理者，每个处理者只关心自己的职责。
//! - 调用方只把请求交给链头，不需要知道认证、缓存、路由分别由谁处理。
//!
//! 场景说明：
//! - `AuthHandler` 校验 token，失败时直接终止链路。
//! - `CacheHandler` 命中缓存时直接返回，未命中时继续传递。
//! - `RouteHandler` 是链尾，负责生成最终响应。

fn main() {
    let chain = AuthHandler::new(Some(Box::new(CacheHandler::new(Some(Box::new(
        RouteHandler,
    ))))));

    let requests = [
        Request::new("/api/users", Some("valid-token")),
        Request::new("/api/cached/config", Some("valid-token")),
        Request::new("/api/orders", None),
    ];

    for request in requests {
        let response = chain.handle(&request);
        println!(
            "[main] path={} status={} body={}",
            request.path, response.status, response.body
        );
    }
}

/// Web 请求：沿责任链传递的数据对象。
#[derive(Debug)]
struct Request {
    path: String,
    token: Option<String>,
}

impl Request {
    /// 创建一个示例请求。
    fn new(path: &str, token: Option<&str>) -> Self {
        Self {
            path: path.to_string(),
            token: token.map(str::to_string),
        }
    }
}

/// Web 响应：每个处理者都可以决定是否直接返回响应。
#[derive(Debug)]
struct Response {
    status: u16,
    body: String,
}

impl Response {
    /// 创建响应对象。
    fn new(status: u16, body: impl Into<String>) -> Self {
        Self {
            status,
            body: body.into(),
        }
    }
}

/// 处理者接口：决定处理请求，或把请求交给下一个处理者。
trait Handler {
    /// 处理请求并返回响应。
    fn handle(&self, request: &Request) -> Response;
}

/// 认证处理者：只处理 token 校验，其余工作交给后继节点。
struct AuthHandler {
    next: Option<Box<dyn Handler>>,
}

impl AuthHandler {
    /// 创建认证处理者，并可选绑定下一个处理者。
    fn new(next: Option<Box<dyn Handler>>) -> Self {
        Self { next }
    }
}

impl Handler for AuthHandler {
    fn handle(&self, request: &Request) -> Response {
        println!("[auth] check token for {}", request.path);
        if request.token.as_deref() != Some("valid-token") {
            return Response::new(401, "unauthorized");
        }

        forward_or_not_found(&self.next, request)
    }
}

/// 缓存处理者：命中缓存时直接响应，否则继续传递。
struct CacheHandler {
    next: Option<Box<dyn Handler>>,
}

impl CacheHandler {
    /// 创建缓存处理者，并可选绑定下一个处理者。
    fn new(next: Option<Box<dyn Handler>>) -> Self {
        Self { next }
    }
}

impl Handler for CacheHandler {
    fn handle(&self, request: &Request) -> Response {
        println!("[cache] lookup {}", request.path);
        if request.path.contains("cached") {
            return Response::new(200, "response from cache");
        }

        forward_or_not_found(&self.next, request)
    }
}

/// 路由处理者：链尾的业务处理节点。
struct RouteHandler;

impl Handler for RouteHandler {
    fn handle(&self, request: &Request) -> Response {
        println!("[route] route {}", request.path);
        Response::new(200, format!("route handler processed {}", request.path))
    }
}

fn forward_or_not_found(next: &Option<Box<dyn Handler>>, request: &Request) -> Response {
    match next {
        Some(handler) => handler.handle(request),
        None => Response::new(404, "no handler"),
    }
}
