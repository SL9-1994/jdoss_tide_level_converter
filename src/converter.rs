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
        let mut chars = txt_data.chars().collect::<Vec<_>>();
        let mut i = 0;
        while i < chars.len() - 3 {
            if chars[i] == '9' && chars[i + 1] == '9' && chars[i + 2] == '9' && chars[i + 3] == '9'
            {
                if i != 0 && chars[i - 1] != '\n' {
                    chars.insert(i, ' ');
                    i += 5; // 4つの9と新しく挿入したスペースをスキップ
                } else {
                    i += 4; // 4つの9をスキップ
                }
            } else {
                i += 1;
            }
        }
        chars.into_iter().collect()
    }
}

// 港湾局: 0~9
// 国土地理院: GS
// 気象庁: MA
// 海上保安庁: HD
