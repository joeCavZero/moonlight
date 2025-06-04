pub trait Stringable {
    fn processed_string(&self) -> String;

    fn from_bin_to_u8(&self) -> Result<u8, String>;
    fn from_bin_to_i8(&self) -> Result<i8, String>;
    fn from_bin_to_u16(&self) -> Result<u16, String>;
    fn from_bin_to_i16(&self) -> Result<i16, String>;

    fn from_hex_to_u8(&self) -> Result<u8, String>;
    fn from_hex_to_i8(&self) -> Result<i8, String>;
    fn from_hex_to_u16(&self) -> Result<u16, String>;
    fn from_hex_to_i16(&self) -> Result<i16, String>;
}

impl Stringable for String {
    fn processed_string(&self) -> String {
        let mut result = String::new();
        let mut chars = self.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '\\' {
                match chars.next() {
                    Some('n') => result.push('\n'),
                    Some('t') => result.push('\t'),
                    Some('r') => result.push('\r'),
                    Some('"') => result.push('"'),
                    Some('\\') => result.push('\\'),
                    Some('0') => result.push('\0'),
                    Some(ch) => {
                        // Tratar sequência de escape inválida incluindo-a como está
                        result.push('\\');
                        result.push(ch);
                    },
                    None => {
                        // Sequência incompleta, adiciona a barra invertida
                        result.push('\\');
                    },
                }
            } else {
                result.push(ch);
            }
        }
        
        result
    }

    /////////////////////////////////
    // Binary Conversion
    /////////////////////////////////

    fn from_bin_to_u8(&self) -> Result<u8, String> {
        let lowercased = self.clone().to_lowercase();
        if !lowercased.starts_with("0b") || lowercased.len() < 3 || lowercased.len() > 10 {
            return Err("Invalid binary format. Must start with '0b' and be between 1 and 8 characters long.".to_string());
        }

        let bits = &lowercased[2..];
        let mut res: u8 = 0;
        
        for c in bits.chars() {
            match c {
                '0' => {
                    res = res.wrapping_shl(1);
                }
                '1' => {
                    res = res.wrapping_shl(1) | 1;
                }
                _ => {
                    return Err("Invalid binary digit.".to_string());
                }
            }
        }
    
        Ok(res)

    }

    fn from_bin_to_i8(&self) -> Result<i8, String> {
        let lowercased = self.clone().to_lowercase();
        if !lowercased.starts_with("0b") || lowercased.len() < 3 || lowercased.len() > 10 {
            return Err("Invalid binary format. Must start with '0b' and be between 1 and 8 characters long.".to_string());
        }

        let bits = &lowercased[2..];
        let mut res: i8 = 0;
        
        for c in bits.chars() {
            match c {
                '0' => {
                    res = res.wrapping_shl(1);
                }
                '1' => {
                    res = res.wrapping_shl(1) | 1;
                }
                _ => {
                    return Err("Invalid binary digit.".to_string());
                }
            }
        }
    
        Ok(res)

    }

    fn from_bin_to_u16(&self) -> Result<u16, String> {
        let lowercased = self.clone().to_lowercase();
        if !lowercased.starts_with("0b") || lowercased.len() < 3 || lowercased.len() > 18 {
            return Err("Invalid binary format. Must start with '0b' and be between 1 and 16 characters.".to_string());
        }

        let bits = &lowercased[2..];
        let mut res: u16 = 0;
        
        for c in bits.chars() {
            match c {
                '0' => {
                    res = res.wrapping_shl(1);
                }
                '1' => {
                    res = res.wrapping_shl(1) | 1;
                }
                _ => {
                    return Err("Invalid binary digit.".to_string());
                }
            }
        }
    
        Ok(res)

    }
    
    fn from_bin_to_i16(&self) -> Result<i16, String> {
        let lowercased = self.clone().to_lowercase();
        if !lowercased.starts_with("0b") || lowercased.len() < 3 || lowercased.len() > 18 {
            return Err("Invalid binary format. Must start with '0b' and be between 1 and 16 characters.".to_string());
        }

        let bits = &lowercased[2..];
        let mut res: i16 = 0;
        
        for c in bits.chars() {
            match c {
                '0' => {
                    res = res.wrapping_shl(1);
                }
                '1' => {
                    res = res.wrapping_shl(1) | 1;
                }
                _ => {
                    return Err("Invalid binary digit.".to_string());
                }
            }
        }
    
        Ok(res)

    }

    /////////////////////////////////
    // Hexadecimal Conversion
    /////////////////////////////////
    
    fn from_hex_to_u8(&self) -> Result<u8, String> {
        let lowercased = self.clone().to_lowercase();
        if !lowercased.starts_with("0x") || lowercased.len() < 3 || lowercased.len() > 6 {
            return Err("Invalid hexadecimal format. Must start with '0x' and be between 1 and 4 characters.".to_string());
        }

        let hex_digits = &lowercased[2..];
        let mut res: u8 = 0;
        for c in hex_digits.chars() {
            let digit = match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'a' => 10,
                'b' => 11,
                'c' => 12,
                'd' => 13,
                'e' => 14,
                'f' => 15,
                _ => return Err("Invalid hexadecimal digit.".to_string()),
            };
            match res.checked_mul(16) {
                Some(v) => {
                    match v.checked_add(digit) {
                        Some(v) => res = v,
                        None => return Err("Value overflow while converting hexadecimal to unsigned 8-bit integer.".to_string()),
                    }
                }
                None => return Err("Value overflow while converting hexadecimal to unsigned 8-bit integer.".to_string()),
            }
        }
        Ok(res)
    }

    fn from_hex_to_i8(&self) -> Result<i8, String> {
        let lowercased = self.clone().to_lowercase();
        if !lowercased.starts_with("0x") || lowercased.len() < 3 || lowercased.len() > 6 {
            return Err("Invalid hexadecimal format. Must start with '0x' and be between 1 and 4 characters.".to_string());
        }

        let hex_digits = &lowercased[2..];
        let mut res: i8 = 0;
        for c in hex_digits.chars() {
            let digit = match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'a' => 10,
                'b' => 11,
                'c' => 12,
                'd' => 13,
                'e' => 14,
                'f' => 15,
                _ => return Err("Invalid hexadecimal digit.".to_string()),
            };
            match res.checked_mul(16) {
                Some(v) => {
                    match v.checked_add(digit) {
                        Some(v) => res = v,
                        None => return Err("Value overflow while converting hexadecimal to signed 8-bit integer.".to_string()),
                    }
                }
                None => return Err("Value overflow while converting hexadecimal to signed 8-bit integer.".to_string()),
            }
        }
        Ok(res)
    }

    fn from_hex_to_u16(&self) -> Result<u16, String> {
        let lowercased = self.clone().to_lowercase();
        if !lowercased.starts_with("0x") || lowercased.len() < 3 || lowercased.len() > 6 {
            return Err("Invalid hexadecimal format. Must start with '0x' and be between 1 and 4 characters.".to_string());
        }

        let hex_digits = &lowercased[2..];
        let mut res: u16 = 0;
        for c in hex_digits.chars() {
            let digit = match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'a' => 10,
                'b' => 11,
                'c' => 12,
                'd' => 13,
                'e' => 14,
                'f' => 15,
                _ => return Err("Invalid hexadecimal digit.".to_string()),
            };
            match res.checked_mul(16) {
                Some(v) => {
                    match v.checked_add(digit) {
                        Some(v) => res = v,
                        None => return Err("Value overflow while converting hexadecimal to unsigned 16-bit integer.".to_string()),
                    }
                }
                None => return Err("Value overflow while converting hexadecimal to unsigned 16-bit integer.".to_string()),
            }
        }
        Ok(res)

    }

    fn from_hex_to_i16(&self) -> Result<i16, String> {
        let lowercased = self.clone().to_lowercase();
        if !lowercased.starts_with("0x") || lowercased.len() < 3 || lowercased.len() > 6 {
            return Err("Invalid hexadecimal format. Must start with '0x' and be between 1 and 4 characters.".to_string());
        }

        let hex_digits = &lowercased[2..];
        let mut res: i16 = 0;
        for c in hex_digits.chars() {
            let digit = match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'a' => 10,
                'b' => 11,
                'c' => 12,
                'd' => 13,
                'e' => 14,
                'f' => 15,
                _ => return Err("Invalid hexadecimal digit.".to_string()),
            };
            match res.checked_mul(16) {
                Some(v) => {
                    match v.checked_add(digit) {
                        Some(v) => res = v,
                        None => return Err("Value overflow while converting hexadecimal to signed 16-bit integer.".to_string()),
                    }
                }
                None => return Err("Value overflow while converting hexadecimal to signed 16-bit integer.".to_string()),
            }
        }

        Ok(res)

    }

    

}
