pub struct Converter;

/// The `Converter` struct provides methods for converting illegal text data to normal text data.
impl Converter {
    /// Creates a new `Converter` instance.
    pub fn new() -> Self {
        Self {}
    }

    /// Converts incorrect tidal text data to normal text data.
    ///
    /// ## Arguments
    ///
    /// * `txt_data` - The input text data to be converted.
    ///
    /// ## Returns
    ///
    /// * The converted text data.
    pub fn convert_illegal_to_normal(&self, txt_data: String) -> String {
        let mut result = String::new();
        for line in txt_data.lines() {
            // 20個連続のスペースがあるかどうかをチェック
            if line.chars().take(20).all(|c| c == ' ') {
                continue; // スペースが20個連続している行はスキップ
            }

            let mut chars = line.chars().collect::<Vec<_>>();
            let mut i = 0;
            while i < chars.len() - 3 {
                if chars[i..i + 20].iter().all(|&c| c == ' ') {
                    break; // 20個のスペースが見つかったら、その行の残りをスキップ
                }
                if chars[i] == '9'
                    && chars[i + 1] == '9'
                    && chars[i + 2] == '9'
                    && chars[i + 3] == '9'
                {
                    if i + 4 >= chars.len() || chars[i + 4] != ' ' {
                        chars.insert(i + 4, ' '); // 4つの9の後にスペースを挿入
                    }
                    i += 5; // 4つの9と新しく挿入したスペースをスキップ
                } else if chars[i].is_digit(10)
                    && chars[i + 1].is_digit(10)
                    && chars[i + 2].is_digit(10)
                    && (i + 3 >= chars.len() || chars[i + 3] != '-')
                {
                    if i + 3 >= chars.len() || chars[i + 3] != ' ' {
                        chars.insert(i + 3, ' '); // 3つの数字の後にスペースを挿入
                    }
                    i += 4; // 3つの数字と新しく挿入したスペースをスキップ
                } else {
                    i += 1;
                }
            }

            result.push_str(&chars.into_iter().collect::<String>());
            result.push('\n');
        }

        result
    }
}

// 港湾局: 0~9
// 国土地理院: GS
// 気象庁: MA
// 海上保安庁: HD
