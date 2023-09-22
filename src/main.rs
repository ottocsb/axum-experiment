//外部引用 dotenv
extern crate dotenv;

// 引入sqlx mysql特征 引入dotenv和env 引入axum 引入serde 用于序列化和反序列化
use dotenv::dotenv;
use std::env;
use axum::{
    routing::{get, post},
    response::IntoResponse,
    extract::Path,
    Router, Json,
};
use sqlx::{
    mysql::MySqlPoolOptions,
    MySql, Pool,
};

use serde::{Deserialize, Serialize};

// 引入rand_n 和 unit 模块
// mod rand_n;
// use rand_n::rand_id;
mod unit;
mod error;
mod response;

pub use response::Response;
use crate::error::{AppError, AppErrorType};

type Result<T> = std::result::Result<T, AppError>;

async fn get_client() -> Result<Pool<MySql>> {
    // 从环境变量中获取数据库连接地址
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // 连接数据库
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(database_url.as_str())
        .await
        .map_err(|err| AppError {
            message: Some(err.to_string()),
            cause: None,
            error_type: AppErrorType::DbType, // You can choose the appropriate error type here
        })?;
    // 返回连接池
    Ok(pool)
}

#[tokio::main]
async fn main() {
    // 加载.env文件
    dotenv().ok();

    // let load_data =load_data().await.unwrap();
    // let length = load_data.len();
    // println!("{:?}",length);

    // 构建router
    let app = Router::new()
        .route("/getUser/:id", get(get_user))
        .route("/users", post(create_user));

    // 运行hyper  http服务 localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// handler对应的函数
async fn get_user(Path(user_id): Path<u32>) -> impl IntoResponse {
    let pool = match get_client().await {
        Ok(pool) => pool,
        Err(err) => {
            return err.into_response();
        }
    };

    // 查询一条数据
    let id = user_id;
    let user_info = sqlx::query!(r#"SELECT * FROM user_t WHERE id = ?"#, id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            println!("error: {}", e);
            e
        });

    match user_info {
        Ok(Some(user)) => {
            let info = User {
                username: user.name,
                address: user.address,
                id: Some(user.id),
            };
            Json(Response::ok(info))
        }
        _ => Json(Response::err(500, "not found".to_string())),
    }
}


async fn create_user(Json(payload): Json<User>) -> impl IntoResponse {
    let pool = get_client()
        .await
        .unwrap();

    let user = User {
        username: payload.username,
        address: payload.address,
        id: None,
    };
    // 插入一条数据
    let new_user = sqlx::query!(r#"INSERT INTO user_t (name, address) VALUES (?, ?)"#,user.username,user.address)
        .execute(&pool)
        .await
        .map_err(|e| {
            println!("error: {}", e);
            e
        });
    println!("{:?}", new_user);
    (Json(Response::ok(user)))
}


// 定义一个user 
#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: Option<String>,
    address: Option<String>,
    id: Option<i32>,
}
