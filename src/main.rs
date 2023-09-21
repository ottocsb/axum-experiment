//外部引用 dotenv
extern crate dotenv;
// 引入sqlx mysql特征 引入dotenv和env 引入axum 引入serde 用于序列化和反序列化
use sqlx::mysql::MySqlPoolOptions;
use dotenv::dotenv;
use std::env;
use axum::{
    routing::{get, post},
    response::IntoResponse,
    response::Html,
    http::StatusCode,
    extract::Path,
    Router,Json
};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};

// 引入rand_n 和 unit 模块
// mod rand_n;
// use rand_n::rand_id;
// mod unit;
// use unit::load_data;

#[tokio::main]
async fn main() {
    // 加载.env文件
    dotenv().ok();
    // 从环境变量中获取数据库连接地址
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // 连接数据库
    let pool = MySqlPoolOptions::new()
        .connect(database_url.as_str())
        .await
        .map_err(|e| { println!("error: {}", e);e })?;

    // let load_data =load_data().await.unwrap();
    // let length = load_data.len();
    // println!("{:?}",length);

    // 构建router
    let app = Router::new()
    .route("/", get(root))  //路径对应handler
    .route("/getUser/:id", get(get_user)).layer(pool.clone())
    .route("/users", post(create_user)).layer(pool.clone());

    // 运行hyper  http服务 localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// handler对应的函数
async fn root()-> Html<&'static str> {
    println!("Hello, world!");
    Html("<p>Hello, Welcome root!</p>")
}

async fn get_user(pool: Pool<MySql>,Path(user_id): Path<u32>)-> String  {
    // 查询一条数据
    let id = user_id ;
    let user_info = sqlx::query!(r#"SELECT * FROM user_t WHERE id = ?"#,id)
        .fetch_all(&pool)
        .await
        .map_err(|e| { println!("error: {}", e);e })?;

    println!("{:?}", user_info);
    format!("Hello, Welcome get_user!{:?}",user_info)
}

async fn create_user( pool:Pool<MySql>,Json(payload): Json<User>,) -> impl IntoResponse {
    let user = User {
        username: payload.username,
        address: payload.address,
        id: None,
    };
    // 插入一条数据
    let new_user = sqlx::query!(r#"INSERT INTO user_t (name, address) VALUES (?, ?)"#,user.username,user.address)
        .execute(&pool)
        .await
        .map_err(|e| { println!("error: {}", e);e })?;
    println!("{:?}",new_user);
    (StatusCode::CREATED, Json(user))
}


// 定义一个user 
#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    address: String,
    id: Option<i32>,
}
