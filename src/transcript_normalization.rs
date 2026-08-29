const TERMINAL_PUNCTUATION: &[char] = &['.', '!', '?', '…', '。', '！', '？', '｡'];
const TRAILING_CLOSERS: &[char] = &[
    '"', '\'', '”', '’', '»', ')', ']', '}', '）', '］', '｝', '」', '』', '】',
];

pub fn normalize(text: &str) -> String {
    let text = text.trim();
    if text.is_empty() {
        return String::new();
    }

    let mut normalized = capitalize_first_letter(text);
    append_terminal_period(&mut normalized);
    normalized
}

fn capitalize_first_letter(text: &str) -> String {
    let Some((index, letter)) = text
        .char_indices()
        .find(|(_, character)| character.is_alphabetic())
    else {
        return text.to_string();
    };
    let upper = letter.to_uppercase().collect::<String>();
    if upper == letter.to_string() {
        return text.to_string();
    }
    let mut output = String::with_capacity(text.len() + upper.len());
    output.push_str(&text[..index]);
    output.push_str(&upper);
    output.push_str(&text[index + letter.len_utf8()..]);
    output
}

fn append_terminal_period(text: &mut String) {
    let Some(mut last) = text.char_indices().next_back().map(|(index, _)| index) else {
        return;
    };
    while text[last..]
        .chars()
        .next()
        .is_some_and(|character| TRAILING_CLOSERS.contains(&character))
    {
        let Some((previous, _)) = text[..last].char_indices().next_back() else {
            return;
        };
        last = previous;
    }
    if !text[last..]
        .chars()
        .next()
        .is_some_and(|character| TERMINAL_PUNCTUATION.contains(&character))
    {
        text.push('.');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capitalizes_and_terminates_latin_transcripts() {
        assert_eq!(normalize("hello world"), "Hello world.");
        assert_eq!(normalize("  \"hello world\"  "), "\"Hello world\".");
    }

    #[test]
    fn preserves_existing_terminal_punctuation_and_japanese_text() {
        assert_eq!(normalize("already done!"), "Already done!");
        assert_eq!(normalize("日本語です。"), "日本語です。");
        assert_eq!(normalize("「日本語です。」"), "「日本語です。」");
    }

    #[test]
    fn inserts_period_after_a_closing_character() {
        assert_eq!(normalize("hello world)"), "Hello world).");
    }

    #[test]
    fn handles_empty_and_non_letter_text() {
        assert_eq!(normalize("   "), "");
        assert_eq!(normalize("123"), "123.");
    }
}
