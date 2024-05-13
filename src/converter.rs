pub struct Converter;

impl Converter {
    /// Creates a new `Converter` instance.
    pub fn new() -> Self {
        Self {}
    }

    /// Converts missing digits to normal digits in the given text data.
    ///
    /// ## Arguments
    ///
    /// * `txt_data` - The text data to convert.
    ///
    /// ## Returns
    ///
    /// * The converted text data with missing digits replaced by normal digits.
    pub fn convert_missing_to_normal(&self, txt_data: String) -> String {
        let mut result = String::new();

        for (_index, line) in txt_data.lines().enumerate() {
            // 行の反転を行い、探索方向を逆順にする
            let reversed_line: String = line.chars().rev().collect();

            // 行の中に偶数桁で連続している9があったら、4桁ごと(前に)にスペースを挿入
            let mut converted_line = String::new();
            let mut count = 0;
            let mut chars = reversed_line.chars().peekable();
            // 各行の文字を逆順で探索を行う
            while let Some(c) = chars.next() {
                if c == '9' {
                    count += 1;
                } else {
                    count = 0;
                }
                converted_line.push(c);
                if count % 4 == 0 && count > 0 && chars.peek().is_some() {
                    converted_line.push(' ');
                }
            }

            result.push_str(&converted_line.chars().rev().collect::<String>());
            result.push('\n');
        }

        result
    }
}
