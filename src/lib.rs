// Copyright 2024 itpey
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;

/// The base64 URL-safe charset used for encoding.
const BASE64_URL_CHARSET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

lazy_static::lazy_static! {
    static ref BASE64_URL_CHARSET_INDEX: HashMap<char, u64> = {
        BASE64_URL_CHARSET.chars().enumerate().map(|(i, c)| (c, i as u64)).collect()
    };
}

/// ToShiny converts an id to a shiny.
///
/// # Arguments
///
/// * `id` - The ID to convert.
///
/// # Returns
///
/// shiny.
///
/// # Examples
///
/// ```
/// use shinyid::{to_shiny};
/// let shiny = to_shiny(500);
/// assert_eq!(shiny, "H0");
/// ```
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

/// ToId converts a shiny to its corresponding id.
///
/// # Arguments
///
/// * `shiny` - The shiny to convert.
///
/// # Returns
///
/// The ID, or an error if the shiny is invalid.
///
/// # Examples
///
/// ```
/// use shinyid::{to_id};
/// let id = to_id("H0").unwrap();
/// assert_eq!(id, 500);
/// ```
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
