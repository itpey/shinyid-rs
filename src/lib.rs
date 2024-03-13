use std::collections::HashMap;

const BASE64_URL_CHARSET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

lazy_static::lazy_static! {
    static ref BASE64_URL_CHARSET_INDEX: HashMap<char, u64> = {
        BASE64_URL_CHARSET.chars().enumerate().map(|(i, c)| (c, i as u64)).collect()
    };
}

pub fn to_shiny(mut id: u64) -> String {
    if id == 0 {
        return "A".to_string();
    }

    let mut shiny = String::with_capacity(11);
    while id > 0 {
        let remainder = id % 64;
        shiny.push(BASE64_URL_CHARSET.as_bytes()[remainder as usize] as char);
        id /= 64;
    }
    shiny.chars().rev().collect()
}

pub fn to_id(shiny: &str) -> Result<u64, &'static str> {
    if shiny == "A" {
        return Ok(0);
    }

    if !is_valid_shiny(shiny) {
        return Err("Input must be a valid shiny");
    }

    let mut base10 = 0;
    for &c in shiny.as_bytes() {
        let base64_value = *BASE64_URL_CHARSET_INDEX.get(&(c as char)).unwrap();
        base10 = (base10 << 6) | base64_value;
    }

    Ok(base10)
}

fn is_valid_shiny(shiny: &str) -> bool {
    shiny
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_shiny_and_back() {
        let tests = [
            (0, "A"),
            (62, "-"),
            (63, "_"),
            (1, "B"),
            (500, "H0"),
            (9375, "CSf"),
            (18446744073709551615, "P__________"),
        ];

        for &(id, expected_code) in tests.iter() {
            let result = to_shiny(id);
            assert_eq!(result, expected_code);
            let parsed_id = to_id(&result).unwrap();
            assert_eq!(id, parsed_id);
        }
    }

    #[test]
    fn test_is_valid_shiny() {
        let tests = [
            ("A", true),
            ("b", true),
            ("_", true),
            ("-", true),
            ("F_0", true),
            ("fPg97-", true),
            ("!@#$%", false),
            ("1_1-Q_@-", false),
        ];

        for &(code, expected_bool) in tests.iter() {
            let result = is_valid_shiny(code);
            assert_eq!(result, expected_bool);
        }
    }
}
