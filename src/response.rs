use serde::Serialize;

/// 返回体
/// example:
/// ```json
/// {
///   "code": 0,
///   "msg": "OK",
///   "data": { }
/// }
/// ```

#[derive(Serialize)]
pub struct Response<T: Serialize> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

/// 实现Response
impl<T> Response<T>
    where
        T: Serialize,
{
    pub fn new(code: i32, msg: String, data: Option<T>) -> Self {
        Self { code, msg, data }
    }
    pub fn ok(data: T) -> Self {
        Self::new(0, "OK".to_string(), Some(data))
    }
    pub fn err(code: i32, msg: String) -> Self {
        Self::new(code, msg, None)
    }
}