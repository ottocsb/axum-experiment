use calamine::{open_workbook, Xlsx, Reader};
use rand::seq::SliceRandom;
use std::error::Error as StdError;
use std::fs::read_to_string;

pub async fn load_data() -> Result<Vec<Vec<String>>, Box<dyn StdError>> {
    let file_name = "users.xlsx";
    let mut excel: Xlsx<_> = open_workbook(file_name)?;

    let mut data = Vec::new();

    if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
        for row in r.rows() {
            // 跳过第一行
            if row[0].get_string().unwrap() == "工号" {
                continue;
            }
            let row_data = row.iter().map(|cell| cell.to_string()).collect();
            data.push(row_data);
        }
        let mut rng = rand::thread_rng();
        data.shuffle(&mut rng);
    }

    let cache_data = load_temp_data().await;
    println!("{:?}", cache_data);
    if cache_data.is_empty() || cache_data.iter().any(|s| s.is_empty()) {
        // 执行一些操作
        println!("都为空")
    }
    Ok(data)
}


async fn load_temp_data() -> Vec<String> {
    let mut pros = vec![];

    let temp_json = {
        match read_to_string("temp.json") {
            Ok(data) => data,
            Err(_) => String::new(),
        }
    };

    let error_json = {
        match read_to_string("error.json") {
            Ok(data) => data,
            Err(_) => String::new(),
        }
    };

    pros.push(temp_json);
    pros.push(error_json);

    pros
}
