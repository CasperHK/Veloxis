use salvo::prelude::*;
use salvo::serve_static::StaticDir;
use shared::Greeting;

const DIST_DIR: &str = "frontend/dist";

#[handler]
async fn app_shell(res: &mut Response) {
    let greeting = Greeting::demo();
    let bootstrap = serde_json::to_string(&greeting).unwrap_or_else(|_| "null".to_owned());
    let html = format!(
        r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Veloxis</title>
  </head>
  <body>
    <div id="root"></div>
    <script id="boot-data" type="application/json">{bootstrap}</script>
    <script type="module" src="/frontend.js"></script>
  </body>
</html>"#
    );

    res.render(Text::Html(html));
}

#[handler]
async fn greeting_api(res: &mut Response) {
    res.render(Json(Greeting::demo()));
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .push(Router::with_path("api/greeting").get(greeting_api))
        .get(app_shell)
        .push(Router::with_path("<**path>").get(StaticDir::new([DIST_DIR]).defaults("index.html")));

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}
