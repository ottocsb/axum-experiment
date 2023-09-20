mod rand_n;
use rand_n::rand_id;
// mod unit;
// use unit::load_data;
use axum::{
    routing::{get, post},
    response::IntoResponse,
    response::Html,
    http::StatusCode,
    Router,Json
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    
    // let load_data =load_data().await.unwrap();
    // let length = load_data.len();
    // println!("{:?}",length);
    // 构建router
    let app = Router::new()
    .route("/", get(root))  //路径对应handler
    .route("/getData", post(get_data))
    .route("/foo", get(get_foo).post(post_foo))
    .route("/users", post(create_user));

    // 运行hyper  http服务 localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

}

// 一个个handler
async fn root()-> Html<&'static str> {
    println!("Hello, world!");
    Html("<p>Hello, World!</p>")
}
async fn get_foo() {
    println!("get_foo");
}
async fn post_foo() {
    println!("post_foo");
}
async fn get_data() {
    println!("get_data");
}
async fn create_user( Json(payload): Json<CreateUser>,) -> impl IntoResponse {
    let user = User {
        id :rand_id(12),
        username: payload.username,
    };
    println!("new User: {}", user);
    (StatusCode::CREATED, Json(user))
}


// 定义一个createUser
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}
// 定义一个user 
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, " {{ id: {}, username: {} }}", self.id, self.username)
    }
}

