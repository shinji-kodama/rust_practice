use actix_web::{web, App, HttpResponse, HttpServer};
// useでSerdeの型をインポート
use serde::{Deserialize, Serialize};

// deriveアトリビュートでDeserializeを指定し、コンパイル時にtraitを自動生成
#[derive(Deserialize)]
struct HelloQuery {
    name: String,
    age: u32,
}

// Serializeをderive　JSONに変換するtraitを自動生成
#[derive(Serialize)]
struct HelloResponse {
    greet: String,
}

#[actix_web::get("/")]
async fn hello(
    query: web::Query<HelloQuery>,  // 仮引数にクエリ -> リクエストパラメータを引数で受け取る
) -> HttpResponse {
    let query = query.into_inner(); // クエリから値を取り出す
    let message = format!(          // 文字列メッセージを作成
        "Hello, my name is {}! I am {} years old!",
        query.name, query.age
    );
    let h = HelloResponse { greet: message }; // HelloResponse型にレスポンスするメッセージ入れ
		
    HttpResponse::Ok().json(h) // 200 OKのレスポンスに渡す
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}