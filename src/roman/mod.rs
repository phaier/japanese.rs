mod kana;
mod romans;

pub use kana::to_kana;
pub use romans::to_romans;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_conversion() {
        let test_cases: Vec<(&str, Vec<&str>)> = vec![
            ("", vec![""]),
            ("あ", vec!["a"]),
            ("い", vec!["i"]),
            ("う", vec!["u"]),
            ("え", vec!["e"]),
            ("お", vec!["o"]),
            ("か", vec!["ka"]),
            ("き", vec!["ki"]),
            ("く", vec!["ku"]),
            ("け", vec!["ke"]),
            ("こ", vec!["ko"]),
            ("さ", vec!["sa"]),
            ("し", vec!["si"]),
            ("す", vec!["su"]),
            ("せ", vec!["se"]),
            ("そ", vec!["so"]),
            ("た", vec!["ta"]),
            ("ち", vec!["ti"]),
            ("つ", vec!["tu"]),
            ("て", vec!["te"]),
            ("と", vec!["to"]),
            ("な", vec!["na"]),
            ("に", vec!["ni"]),
            ("ぬ", vec!["nu"]),
            ("ね", vec!["ne"]),
            ("の", vec!["no"]),
            ("は", vec!["ha"]),
            ("ひ", vec!["hi"]),
            ("ふ", vec!["hu"]),
            ("へ", vec!["he"]),
            ("ほ", vec!["ho"]),
            ("ま", vec!["ma"]),
            ("み", vec!["mi"]),
            ("む", vec!["mu"]),
            ("め", vec!["me"]),
            ("も", vec!["mo"]),
            ("や", vec!["ya"]),
            ("ゆ", vec!["yu"]),
            ("よ", vec!["yo"]),
            ("ら", vec!["ra"]),
            ("り", vec!["ri"]),
            ("る", vec!["ru"]),
            ("れ", vec!["re"]),
            ("ろ", vec!["ro"]),
            ("わ", vec!["wa"]),
            ("を", vec!["wo"]),
            ("が", vec!["ga"]),
            ("ぎ", vec!["gi"]),
            ("ぐ", vec!["gu"]),
            ("げ", vec!["ge"]),
            ("ご", vec!["go"]),
            ("ざ", vec!["za"]),
            ("じ", vec!["zi"]),
            ("ず", vec!["zu"]),
            ("ぜ", vec!["ze"]),
            ("ぞ", vec!["zo"]),
            ("だ", vec!["da"]),
            ("ぢ", vec!["di"]),
            ("づ", vec!["du"]),
            ("で", vec!["de"]),
            ("ど", vec!["do"]),
            ("ば", vec!["ba"]),
            ("び", vec!["bi"]),
            ("ぶ", vec!["bu"]),
            ("べ", vec!["be"]),
            ("ぼ", vec!["bo"]),
            ("ぱ", vec!["pa"]),
            ("ぴ", vec!["pi"]),
            ("ぷ", vec!["pu"]),
            ("ぺ", vec!["pe"]),
            ("ぽ", vec!["po"]),
            ("きゃ", vec!["kya"]),
            ("きゅ", vec!["kyu"]),
            ("きょ", vec!["kyo"]),
            ("しゃ", vec!["sya"]),
            ("しゅ", vec!["syu"]),
            ("しょ", vec!["syo"]),
            ("ちゃ", vec!["tya"]),
            ("ちゅ", vec!["tyu"]),
            ("ちょ", vec!["tyo"]),
            ("にゃ", vec!["nya"]),
            ("にゅ", vec!["nyu"]),
            ("にょ", vec!["nyo"]),
            ("ひゃ", vec!["hya"]),
            ("ひゅ", vec!["hyu"]),
            ("ひょ", vec!["hyo"]),
            ("みゃ", vec!["mya"]),
            ("みゅ", vec!["myu"]),
            ("みょ", vec!["myo"]),
            ("りゃ", vec!["rya"]),
            ("りゅ", vec!["ryu"]),
            ("りょ", vec!["ryo"]),
            ("ぎゃ", vec!["gya"]),
            ("ぎゅ", vec!["gyu"]),
            ("ぎょ", vec!["gyo"]),
            ("じゃ", vec!["zya"]),
            ("じゅ", vec!["zyu"]),
            ("じょ", vec!["zyo"]),
            ("びゃ", vec!["bya"]),
            ("びゅ", vec!["byu"]),
            ("びょ", vec!["byo"]),
            ("ぴゃ", vec!["pya"]),
            ("ぴゅ", vec!["pyu"]),
            ("ぴょ", vec!["pyo"]),
            ("っか", vec!["kka"]),
            ("ぁ", vec!["la"]),
            ("ぃ", vec!["li"]),
            ("ぅ", vec!["lu"]),
            ("ぇ", vec!["le"]),
            ("ぉ", vec!["lo"]),
            ("ヵ", vec!["lka"]),
            ("あいうえお", vec!["aiueo"]),
            ("かきくけこ", vec!["kakikukeko"]),
            ("さしすせそ", vec!["sasisuseso"]),
            ("たちつてと", vec!["tatituteto"]),
            ("なにぬねの", vec!["naninuneno"]),
            ("はひふへほ", vec!["hahihuheho", "hahifuheho"]),
            ("まみむめも", vec!["mamimumemo"]),
            ("やゆよ", vec!["yayuyo"]),
            ("らりるれろ", vec!["rarirurero"]),
            ("わをん", vec!["wawonn"]),
            ("がぎぐげご", vec!["gagigugego"]),
            ("ざじずぜぞ", vec!["zazizuzezo"]),
            ("だぢづでど", vec!["dadidudedo"]),
            ("ばびぶべぼ", vec!["babibubebo"]),
            ("ぱぴぷぺぽ", vec!["papipupepo"]),
            ("あいうえおかきくけこ", vec!["aiueokakikukeko"]),
            ("きょうとふ", vec!["kyoutohu"]),
            ("こっかいぎじどう", vec!["kokkaigizidou"]),
            ("なんばんとらい", vec!["nanbantorai"]),
            ("これはぺんです", vec!["korehapendesu"]),
            ("これはぺんです", vec!["korehapenndesu"]),
            ("ろーま", vec!["ro-ma"]),
            ("きょうと", vec!["kyouto"]),
            /*
            ("kた", vec!["kta"]),
            ("ktーkt", vec!["kt-kt"]),
            */
        ];

        for (kana, romans) in test_cases {
            // かな → ローマ字の変換テスト
            let actual_romans = to_romans(kana);

            // 重複がないことを確認
            for a in &actual_romans {
                assert_eq!(
                    actual_romans.iter().filter(|ac| *ac == a).count(),
                    1,
                    "重複したローマ字変換結果があります: {}",
                    a
                );
            }

            // 期待する結果が含まれていることを確認
            for &roman in &romans {
                assert!(
                    actual_romans.contains(&roman.to_string()),
                    "期待するローマ字「{}」が「{}」の変換結果に含まれていません",
                    roman,
                    kana
                );
            }

            // ローマ字 → かなの変換テスト
            for &roman in &romans {
                assert_eq!(
                    to_kana(roman),
                    kana,
                    "ローマ字「{}」からかな「{}」への変換が失敗しました",
                    roman,
                    kana
                );
            }
        }
    }
}
