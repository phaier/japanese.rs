// ひらがなとローマ字の変換テーブル
pub const TO_ROMANS_TABLE: &[(&str, &[&[&str]])] = &[
    ("あ", &[&["a"]]),
    ("い", &[&["i"]]),
    ("う", &[&["u"]]),
    ("え", &[&["e"]]),
    ("お", &[&["o"]]),
    ("か", &[&["k", "a"]]),
    ("き", &[&["k", "i"]]),
    ("く", &[&["k", "u"]]),
    ("け", &[&["k", "e"]]),
    ("こ", &[&["k", "o"]]),
    ("さ", &[&["s", "a"]]),
    ("し", &[&["s", "i"]]),
    ("す", &[&["s", "u"]]),
    ("せ", &[&["s", "e"]]),
    ("そ", &[&["s", "o"]]),
    ("た", &[&["t", "a"]]),
    ("ち", &[&["t", "i"]]),
    ("つ", &[&["t", "u"]]),
    ("て", &[&["t", "e"]]),
    ("と", &[&["t", "o"]]),
    ("な", &[&["n", "a"]]),
    ("に", &[&["n", "i"]]),
    ("ぬ", &[&["n", "u"]]),
    ("ね", &[&["n", "e"]]),
    ("の", &[&["n", "o"]]),
    ("は", &[&["h", "a"]]),
    ("ひ", &[&["h", "i"]]),
    ("ふ", &[&["h", "u"], &["f", "u"]]),
    ("へ", &[&["h", "e"]]),
    ("ほ", &[&["h", "o"]]),
    ("ま", &[&["m", "a"]]),
    ("み", &[&["m", "i"]]),
    ("む", &[&["m", "u"]]),
    ("め", &[&["m", "e"]]),
    ("も", &[&["m", "o"]]),
    ("や", &[&["y", "a"]]),
    ("ゆ", &[&["y", "u"]]),
    ("よ", &[&["y", "o"]]),
    ("ら", &[&["r", "a"]]),
    ("り", &[&["r", "i"]]),
    ("る", &[&["r", "u"]]),
    ("れ", &[&["r", "e"]]),
    ("ろ", &[&["r", "o"]]),
    ("わ", &[&["w", "a"]]),
    ("を", &[&["w", "o"]]),
    ("ん", &[&["n", "n"]]),
    ("が", &[&["g", "a"]]),
    ("ぎ", &[&["g", "i"]]),
    ("ぐ", &[&["g", "u"]]),
    ("げ", &[&["g", "e"]]),
    ("ご", &[&["g", "o"]]),
    ("ざ", &[&["z", "a"]]),
    ("じ", &[&["z", "i"]]),
    ("ず", &[&["z", "u"]]),
    ("ぜ", &[&["z", "e"]]),
    ("ぞ", &[&["z", "o"]]),
    ("だ", &[&["d", "a"]]),
    ("ぢ", &[&["d", "i"]]),
    ("づ", &[&["d", "u"]]),
    ("で", &[&["d", "e"]]),
    ("ど", &[&["d", "o"]]),
    ("ば", &[&["b", "a"]]),
    ("び", &[&["b", "i"]]),
    ("ぶ", &[&["b", "u"]]),
    ("べ", &[&["b", "e"]]),
    ("ぼ", &[&["b", "o"]]),
    ("ぱ", &[&["p", "a"]]),
    ("ぴ", &[&["p", "i"]]),
    ("ぷ", &[&["p", "u"]]),
    ("ぺ", &[&["p", "e"]]),
    ("ぽ", &[&["p", "o"]]),
    ("きゃ", &[&["k", "y", "a"]]),
    ("きゅ", &[&["k", "y", "u"]]),
    ("きょ", &[&["k", "y", "o"]]),
    ("しゃ", &[&["s", "y", "a"]]),
    ("しゅ", &[&["s", "y", "u"]]),
    ("しょ", &[&["s", "y", "o"]]),
    ("ちゃ", &[&["t", "y", "a"]]),
    ("ちゅ", &[&["t", "y", "u"]]),
    ("ちょ", &[&["t", "y", "o"]]),
    ("にゃ", &[&["n", "y", "a"]]),
    ("にゅ", &[&["n", "y", "u"]]),
    ("にょ", &[&["n", "y", "o"]]),
    ("ひゃ", &[&["h", "y", "a"]]),
    ("ひゅ", &[&["h", "y", "u"]]),
    ("ひょ", &[&["h", "y", "o"]]),
    ("みゃ", &[&["m", "y", "a"]]),
    ("みゅ", &[&["m", "y", "u"]]),
    ("みょ", &[&["m", "y", "o"]]),
    ("りゃ", &[&["r", "y", "a"]]),
    ("りゅ", &[&["r", "y", "u"]]),
    ("りょ", &[&["r", "y", "o"]]),
    ("ぎゃ", &[&["g", "y", "a"]]),
    ("ぎゅ", &[&["g", "y", "u"]]),
    ("ぎょ", &[&["g", "y", "o"]]),
    ("じゃ", &[&["z", "y", "a"]]),
    ("じゅ", &[&["z", "y", "u"]]),
    ("じょ", &[&["z", "y", "o"]]),
    ("ぢゃ", &[&["d", "y", "a"]]),
    ("ぢゅ", &[&["d", "y", "u"]]),
    ("ぢょ", &[&["d", "y", "o"]]),
    ("びゃ", &[&["b", "y", "a"]]),
    ("びゅ", &[&["b", "y", "u"]]),
    ("びょ", &[&["b", "y", "o"]]),
    ("ぴゃ", &[&["p", "y", "a"]]),
    ("ぴゅ", &[&["p", "y", "u"]]),
    ("ぴょ", &[&["p", "y", "o"]]),
    ("てゃ", &[&["t", "h", "a"]]),
    ("てぃ", &[&["t", "h", "i"]]),
    ("てゅ", &[&["t", "h", "u"]]),
    ("てぇ", &[&["t", "h", "e"]]),
    ("てょ", &[&["t", "h", "o"]]),
    ("でゃ", &[&["d", "h", "a"]]),
    ("でぃ", &[&["d", "h", "i"]]),
    ("でゅ", &[&["d", "h", "u"]]),
    ("でぇ", &[&["d", "h", "e"]]),
    ("でょ", &[&["d", "h", "o"]]),
    ("とぁ", &[&["t", "w", "a"]]),
    ("とぃ", &[&["t", "w", "i"]]),
    ("とぅ", &[&["t", "w", "u"]]),
    ("とぇ", &[&["t", "w", "e"]]),
    ("とぉ", &[&["t", "w", "o"]]),
    ("どぁ", &[&["d", "w", "a"]]),
    ("どぃ", &[&["d", "w", "i"]]),
    ("どぅ", &[&["d", "w", "u"]]),
    ("どぇ", &[&["d", "w", "e"]]),
    ("どぉ", &[&["d", "w", "o"]]),
    ("ふぁ", &[&["f", "a"]]),
    ("ふぃ", &[&["f", "i"]]),
    ("ふぇ", &[&["f", "e"]]),
    ("ふぉ", &[&["f", "o"]]),
    ("ぁ", &[&["x", "a"], &["l", "a"]]),
    ("ぃ", &[&["x", "i"], &["l", "i"]]),
    ("ぅ", &[&["x", "u"], &["l", "u"]]),
    ("ぇ", &[&["x", "e"], &["l", "e"]]),
    ("ぉ", &[&["x", "o"], &["l", "o"]]),
    ("ゃ", &[&["x", "y", "a"], &["l", "y", "a"]]),
    ("ゅ", &[&["x", "y", "u"], &["l", "y", "u"]]),
    ("ょ", &[&["x", "y", "o"], &["l", "y", "o"]]),
    ("っ", &[&["x", "t", "u"], &["l", "t", "u"]]),
    ("ヵ", &[&["x", "k", "a"], &["l", "k", "a"]]),
    ("ー", &[&["-"]]),
];

pub const MINS: &str = "ゃゅょぁぃぅぇぉ";

/// ひらがなの文字列をローマ字に変換します。
///
/// # Arguments
///
/// * `text` - ひらがなの文字列
///
/// # Returns
///
/// ローマ字変換された複数の文字列
pub fn to_romans(text: &str) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut results: Vec<String> = vec![];
    let mut i = 0;

    if len < 1 {
        return vec!["".to_string()];
    }

    while i < len {
        let c = chars[i];
        let next = if i + 1 < len {
            Some(chars[i + 1])
        } else {
            None
        };

        if let Some(next_char) = next {
            if MINS.contains(next_char) {
                // ゃゅょぁぃぅぇぉ
                // が次にくる場合は
                // 先読みの必要がある
                let kana = format!("{}{}", c, next_char);
                if let Some(tabled) = find_in_table(&kana) {
                    results = flat_concat(
                        &results,
                        &tabled.iter().map(|ss| ss.join("")).collect::<Vec<String>>(),
                    );
                    i += 2;
                } else {
                    results = flat_concat(&results, &[c.to_string()]);
                    i += 1;
                }
            } else if c == 'っ' {
                // kokkai
                // こっかい
                if let Some(tabled) = find_in_table(&next_char.to_string()) {
                    let mut heads: Vec<String> = Vec::new();
                    for ss in tabled {
                        let s = ss[0];
                        if !heads.iter().any(|h| h == s) {
                            heads.push(s.to_string());
                        }
                    }
                    results = flat_concat(&results, &heads);
                    i += 1;
                } else {
                    // c === っ
                    // n === xxx
                    if let Some(ts) = find_in_table("っ") {
                        results = flat_concat(
                            &results,
                            &ts.iter().map(|ss| ss.join("")).collect::<Vec<String>>(),
                        );
                    }
                    i += 1;
                }
            } else if c == 'ん' {
                if next_char == 'あ'
                    || next_char == 'い'
                    || next_char == 'う'
                    || next_char == 'え'
                    || next_char == 'お'
                {
                    // ん => nn
                    results = flat_concat(&results, &["nn".to_string()]);
                    i += 1;
                } else {
                    // ん => n, nn
                    results = flat_concat(&results, &["n".to_string(), "nn".to_string()]);
                    i += 1;
                }
            } else {
                if let Some(tabled) = find_in_table(&c.to_string()) {
                    results = flat_concat(
                        &results,
                        &tabled.iter().map(|ss| ss.join("")).collect::<Vec<String>>(),
                    );
                    i += 1;
                } else {
                    results = flat_concat(&results, &[c.to_string()]);
                    i += 1;
                }
            }
        } else {
            // 最後の文字の処理
            if let Some(tabled) = find_in_table(&c.to_string()) {
                results = flat_concat(
                    &results,
                    &tabled.iter().map(|ss| ss.join("")).collect::<Vec<String>>(),
                );
            } else {
                results = flat_concat(&results, &[c.to_string()]);
            }
            i += 1;
        }
    }

    results
}

/// テーブルから指定したキーに対応する値を探します
fn find_in_table(key: &str) -> Option<Vec<Vec<&str>>> {
    for (k, v) in TO_ROMANS_TABLE {
        if k == &key {
            // &[&[&str]] を Vec<Vec<&str>> に変換
            return Some(v.iter().map(|ss| ss.to_vec()).collect());
        }
    }
    None
}

/// 結果の配列を平坦化して結合します
fn flat_concat(results: &[String], additions: &[String]) -> Vec<String> {
    if results.is_empty() {
        return additions.to_vec();
    }

    let mut new_results = Vec::new();
    for r in results {
        for a in additions {
            new_results.push(format!("{}{}", r, a));
        }
    }
    new_results
}
