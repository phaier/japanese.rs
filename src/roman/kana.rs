/// ローマ字の文字が母音かどうかを判定します
pub fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'i' || c == 'u' || c == 'e' || c == 'o'
}

/// 二重子音のリスト
pub const DOUBLE_CONSONANT: &'static [&str] = &[
    "ky", "sy", "ty", "ny", "hy", "my", "ry", "gy", "zy", "by", "py", "ts", "ch", "lk", "lt", "xt",
    "ly", "xy",
];

/// 入力された文字列が二重子音かどうかを判定します
pub fn is_double_consonant(roman: &str) -> bool {
    DOUBLE_CONSONANT.contains(&roman)
}

/// ローマ字からひらがなへの変換テーブル
pub const TO_KANA_TABLE: &'static [(&str, &str)] = &[
    ("a", "あ"),
    ("i", "い"),
    ("u", "う"),
    ("e", "え"),
    ("o", "お"),
    ("ka", "か"),
    ("ki", "き"),
    ("ku", "く"),
    ("ke", "け"),
    ("ko", "こ"),
    ("sa", "さ"),
    ("si", "し"),
    ("su", "す"),
    ("se", "せ"),
    ("so", "そ"),
    ("ta", "た"),
    ("ti", "ち"),
    ("tu", "つ"),
    ("te", "て"),
    ("to", "と"),
    ("na", "な"),
    ("ni", "に"),
    ("nu", "ぬ"),
    ("ne", "ね"),
    ("no", "の"),
    ("ha", "は"),
    ("hi", "ひ"),
    ("hu", "ふ"),
    ("he", "へ"),
    ("ho", "ほ"),
    ("ma", "ま"),
    ("mi", "み"),
    ("mu", "む"),
    ("me", "め"),
    ("mo", "も"),
    ("ya", "や"),
    ("yu", "ゆ"),
    ("yo", "よ"),
    ("ra", "ら"),
    ("ri", "り"),
    ("ru", "る"),
    ("re", "れ"),
    ("ro", "ろ"),
    ("wa", "わ"),
    ("wi", "うぃ"),
    ("wu", "う"),
    ("we", "うぇ"),
    ("wo", "を"),
    ("ga", "が"),
    ("gi", "ぎ"),
    ("gu", "ぐ"),
    ("ge", "げ"),
    ("go", "ご"),
    ("za", "ざ"),
    ("zi", "じ"),
    ("zu", "ず"),
    ("ze", "ぜ"),
    ("zo", "ぞ"),
    ("da", "だ"),
    ("di", "ぢ"),
    ("du", "づ"),
    ("de", "で"),
    ("do", "ど"),
    ("ba", "ば"),
    ("bi", "び"),
    ("bu", "ぶ"),
    ("be", "べ"),
    ("bo", "ぼ"),
    ("pa", "ぱ"),
    ("pi", "ぴ"),
    ("pu", "ぷ"),
    ("pe", "ぺ"),
    ("po", "ぽ"),
    ("kya", "きゃ"),
    ("kyi", "きぃ"),
    ("kyu", "きゅ"),
    ("kye", "きぇ"),
    ("kyo", "きょ"),
    ("sya", "しゃ"),
    ("syi", "しぃ"),
    ("syu", "しゅ"),
    ("sye", "しぇ"),
    ("syo", "しょ"),
    ("tya", "ちゃ"),
    ("tyi", "ちぃ"),
    ("tyu", "ちゅ"),
    ("tye", "ちぇ"),
    ("tyo", "ちょ"),
    ("nya", "にゃ"),
    ("nyi", "にぃ"),
    ("nyu", "にゅ"),
    ("nye", "にぇ"),
    ("nyo", "にょ"),
    ("hya", "ひゃ"),
    ("hyi", "ひぃ"),
    ("hyu", "ひゅ"),
    ("hye", "ひぇ"),
    ("hyo", "ひょ"),
    ("mya", "みゃ"),
    ("myi", "みぃ"),
    ("myu", "みゅ"),
    ("mye", "みぇ"),
    ("myo", "みょ"),
    ("rya", "りゃ"),
    ("ryi", "りぃ"),
    ("ryu", "りゅ"),
    ("rye", "りぇ"),
    ("ryo", "りょ"),
    ("gya", "ぎゃ"),
    ("gyi", "ぎぃ"),
    ("gyu", "ぎゅ"),
    ("gye", "ぎぇ"),
    ("gyo", "ぎょ"),
    ("zya", "じゃ"),
    ("zyi", "じぃ"),
    ("zyu", "じゅ"),
    ("zye", "じぇ"),
    ("zyo", "じょ"),
    ("bya", "びゃ"),
    ("byi", "びぃ"),
    ("byu", "びゅ"),
    ("bye", "びぇ"),
    ("byo", "びょ"),
    ("pya", "ぴゃ"),
    ("pyi", "ぴぃ"),
    ("pyu", "ぴゅ"),
    ("pye", "ぴぇ"),
    ("pyo", "ぴょ"),
    ("fa", "ふぁ"),
    ("fi", "ふぃ"),
    ("fu", "ふ"),
    ("fe", "ふぇ"),
    ("fo", "ふぉ"),
    ("tsa", "つぁ"),
    ("tsi", "つぃ"),
    ("tsu", "つ"),
    ("tse", "つぇ"),
    ("tso", "つぉ"),
    ("cha", "ちゃ"),
    ("chi", "ち"),
    ("chu", "ちゅ"),
    ("che", "ちぇ"),
    ("cho", "ちょ"),
    ("la", "ぁ"),
    ("li", "ぃ"),
    ("lu", "ぅ"),
    ("le", "ぇ"),
    ("lo", "ぉ"),
    ("xa", "ぁ"),
    ("xi", "ぃ"),
    ("xu", "ぅ"),
    ("xe", "ぇ"),
    ("xo", "ぉ"),
    ("lka", "ヵ"),
    ("ltu", "っ"),
    ("xtu", "っ"),
    ("lya", "ゃ"),
    ("lyu", "ゅ"),
    ("lyo", "ょ"),
    ("xya", "ゃ"),
    ("xyu", "ゅ"),
    ("xyo", "ょ"),
    ("-", "ー"),
];

/// 変換テーブルから値を取得する関数
fn get_from_table(key: &str) -> Option<&'static str> {
    TO_KANA_TABLE
        .iter()
        .find(|&&(k, _)| k == key)
        .map(|&(_, v)| v)
}

/// 半角のローマ字の文字列をひらがなの文字列に変換します
///
/// # Arguments
///
/// * `roman` - ローマ字の文字列、半角である必要があります
///
/// # Returns
///
/// ひらがなの文字列
pub fn to_kana(roman: &str) -> String {
    let mut result = String::new();
    let mut temp = String::new();
    let mut i = 0;
    let roman = roman.to_lowercase();
    let chars: Vec<char> = roman.chars().collect();
    let len = chars.len();

    while i < len {
        if len <= i {
            result.push_str(&temp);
            break;
        }

        let c = chars[i];
        let code = c as u32;

        if code < 0x61 || code > 0x7a {
            result.push_str(&temp);
            temp.clear();

            match c {
                '-' => result.push_str("ー"),
                _ => result.push(c),
            }

            i += 1;
            continue;
        }

        // 残りの先頭部分をバッファーする
        temp.push(c);

        if let Some(tabled) = get_from_table(&temp) {
            // テーブルで変換可能 => 変換
            result.push_str(tabled);
            temp.clear();
        }

        if temp.len() == 2 {
            // 二文字で始まっていい子音かどうか？
            if is_double_consonant(&temp) {
                i += 1;
                continue;
            }

            let temp_chars: Vec<char> = temp.chars().collect();

            if temp_chars[0] == 'n' {
                if temp_chars[1] == 'n' {
                    // n + n => 『ん』
                    result.push_str("ん");
                    temp = temp[2..].to_string();
                    i += 1;
                    continue;
                }

                if !is_vowel(temp_chars[1]) {
                    // n + 子音 => 『ん』 + 子音
                    result.push_str("ん");
                    temp = temp[1..].to_string();
                    i += 1;
                    continue;
                }
            }

            if temp_chars[0] == 'n' && !is_vowel(temp_chars[1]) {
                // n + 子音 => 『ん』 + 子音
                result.push_str("ん");
                temp = temp[1..].to_string();
                i += 1;
                continue;
            }

            if temp_chars[0] == temp_chars[1] && !is_vowel(temp_chars[0]) {
                // 同一子音 * 2 => 『っ』 + 子音
                result.push_str("っ");
                temp = temp[1..].to_string();
                i += 1;
                continue;
            }

            if !is_vowel(temp_chars[0]) && !is_vowel(temp_chars[1]) {
                result.push(temp_chars[0]);
                temp = temp[1..].to_string();
                i += 1;
                continue;
            }
        }

        if temp.len() == 3 {
            let temp_chars: Vec<char> = temp.chars().collect();
            result.push(temp_chars[0]);
            temp = temp[1..].to_string();
            i += 1;
            continue;
        }

        i += 1;
    }

    result
}
