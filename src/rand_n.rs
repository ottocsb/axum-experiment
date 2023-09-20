use rand::Rng;

// 接收一个usize类型的参数size，返回一个u64类型的随机数
pub fn rand_id(size:usize)->u64 {
    let mut rng = rand::thread_rng();
    let min_value = 10u64.pow(size as u32 - 1);
    let max_value = 10u64.pow(size as u32) - 1;
    rng.gen_range(min_value..=max_value)
}

// 测试用例
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rand_id() {
        let id = rand_id(12);
        println!("{}", id);
        assert!(id >= 100_000_000_000 && id <= 999_999_999_999);
    }
}
