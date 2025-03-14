pub mod hankaku;

/// テキストに含まれる全てのカタカナを平仮名に変換します
/// @param text 入力のテキスト
pub fn katakana_to_hiragana(text: &str) -> String {
    text.chars()
        .map(|c| {
            if ('\u{30A1}'..='\u{30F6}').contains(&c) {
                std::char::from_u32(c as u32 - 0x60).unwrap()
            } else {
                c
            }
        })
        .collect()
}

/// テキストに含まれる全てのひらがなをカタカナに変換します
/// @param text 入力のテキスト
pub fn hiragana_to_katakana(text: &str) -> String {
    text.chars()
        .map(|c| {
            if ('\u{3041}'..='\u{3096}').contains(&c) {
                std::char::from_u32(c as u32 + 0x60).unwrap()
            } else {
                c
            }
        })
        .collect()
}

pub fn to_hiragana(text: &str) -> String {
    katakana_to_hiragana(text)
}

pub fn to_katakana(text: &str) -> String {
    hiragana_to_katakana(text)
}

/// テキストに含まれる全ての文字がひらがなかどうか
/// @param text 入力のテキスト
pub fn is_all_hiragana(text: &str) -> bool {
    text.chars()
        .all(|c| ('\u{3041}'..='\u{3096}').contains(&c) || c == '\u{30FC}')
}

/// テキストに含まれる全ての文字がカタカナかどうか
/// @param text 入力のテキスト
pub fn is_all_katakana(text: &str) -> bool {
    text.chars()
        .all(|c| ('\u{30A1}'..='\u{30F6}').contains(&c) || c == '\u{30FC}')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_hiragana_is_all_hiragana() {
        let cases = vec![("ローカル", true)];
        for (source, result) in cases {
            assert_eq!(is_all_hiragana(&to_hiragana(source)), result);
        }
    }

    #[test]
    fn test_hiragana_katakana_conversion() {
        let cases = vec![
            ("", ""),
            ("あいうえお", "アイウエオ"),
            ("かきくけこ", "カキクケコ"),
            ("さしすせそ", "サシスセソ"),
            ("たちつてと", "タチツテト"),
            ("はひふへほ", "ハヒフヘホ"),
            ("まみむめも", "マミムメモ"),
            ("やゆよ", "ヤユヨ"),
            ("らりるれろ", "ラリルレロ"),
            ("わをん", "ワヲン"),
            ("がぎぐげご", "ガギグゲゴ"),
            ("ざじずぜぞ", "ザジズゼゾ"),
            ("だぢづでど", "ダヂヅデド"),
            ("ばびぶべぼ", "バビブベボ"),
            ("ぱぴぷぺぽ", "パピプペポ"),
            ("ゃゅょぁぃぅぇぉっ", "ャュョァィゥェォッ"),
            ("ゔ", "ヴ"),
        ];
        for (hiragana, katakana) in cases {
            assert_eq!(hiragana_to_katakana(hiragana), katakana);
            assert_eq!(katakana_to_hiragana(katakana), hiragana);
        }
    }

    #[test]
    fn test_is_all_hiragana() {
        let cases = vec![
            ("", true),
            ("あいうえお", true),
            ("かきくけこ", true),
            ("さしすせそ", true),
            ("たちつてと", true),
            ("なにぬねの", true),
            ("はひふへほ", true),
            ("まみむめも", true),
            ("やゆよ", true),
            ("らりるれろ", true),
            ("わをん", true),
            ("ろーま", true),
            ("漢字あいうえお", false),
            ("あいうえお漢字", false),
            ("漢字あいうえお漢字", false),
            ("あいうえおアイウエオ", false),
            ("アイウエオあいうえお", false),
            ("アイウエオあいうえおアイウエオ", false),
        ];
        for (text, result) in cases {
            assert_eq!(is_all_hiragana(text), result);
        }
    }

    #[test]
    fn test_is_all_katakana() {
        let cases = vec![
            ("", true),
            ("アイウエオ", true),
            ("ローマ", true),
            ("漢字アイウエオ", false),
            ("アイウエオ漢字", false),
            ("漢字アイウエオ漢字", false),
            ("アイウエオあいうえお", false),
            ("あいうえおアイウエオ", false),
            ("あいうえおアイウエオあいうえお", false),
        ];
        for (text, result) in cases {
            assert_eq!(is_all_katakana(text), result);
        }
    }
}
