pub fn encrypt(salt: &str, text: &str) -> String {
    text.chars()
        .map(|t| {
            format!("{}{}", t, salt)
                .chars()
                .map(|s| s as u32)
                .reduce(|a, b| a ^ b)
                .unwrap()
        })
        .map(|t| format!("{:x}", t))
        .collect::<Vec<String>>()
        .join("-")
}

pub fn decrypt(salt: &str, text: &str) -> String {
    text.split('-')
        .map(|t| char::from_u32(u32::from_str_radix(t, 16).unwrap()).unwrap())
        .map(|t| {
            format!("{}{}", t, salt)
                .chars()
                .map(|s| s as u32)
                .reduce(|a, b| a ^ b)
                .unwrap()
        })
        .map(|t| char::from_u32(t).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::crypt::{decrypt, encrypt};

    #[test]
    fn check() {
        let t1 = encrypt("123", "中国");
        let t2 = decrypt("123", &t1);
        assert_eq!("中国", t2);
    }
}
