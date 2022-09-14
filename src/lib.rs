pub fn transform(input: &str, line_width: u32) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::transform;

    #[test]
    fn simple() {
        let lorem_ipsum = "Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua";
        let test_cases = [
            ("", 5, ""),
            ("test", 4, "test"),
            ("test", 5, "test "),
            // Not sure about this, maybe we're not supposed to break words.
            // Maybe we're supposed to just panic.
            // Also if a word can't fit in one line, do we always put its beginning on a new line?
            // LibreOffice Writer seems to act this way.
            ("test", 1, "t\ne\ns\nt"),
            ("  12345 ", 2, "12\n34\n5 "),
            ("  ", 5, ""),
            ("a   a  123", 4, "a  a\n123 "),
            ("aaa aaa", 5, "aaa  \naaa  "), // Non-breaking space instead of a regular one. What do?
            ("a a a 12345", 8, "a   a  a\n12345   "),
            ("1234567", 3, "123\n456\n7  "),
            ("12 123456789abc 1", 5, "12   \n12345\n6789a\nbc  1"),

            (lorem_ipsum, 12,
             "Lorem  ipsum\ndolor    sit\namet        \nconsectetur \nadipiscing  \nelit  sed do\neiusmod     \ntempor      \nincididunt  \nut labore et\ndolore magna\naliqua      "),
            (lorem_ipsum, 7,
             "Lorem  \nipsum  \ndolor  \nsit    \namet   \nconsect\netur   \nadipisc\ning    \nelit   \nsed  do\neiusmod\ntempor \nincidid\nunt  ut\nlabore \net     \ndolore \nmagna  \naliqua "),
            (lorem_ipsum, 35,
             "Lorem    ipsum   dolor   sit   amet\nconsectetur  adipiscing elit sed do\neiusmod tempor incididunt ut labore\net      dolore     magna     aliqua"),
        ];

        for &(input, line_width, expected) in &test_cases {
            assert_eq!(
                transform(input, line_width), expected,
                "input: \"{}\", width: {}", input, line_width
            );
        }
    }
    // TODO refactor: add randomized tests with algorithmic correctness checks (i.e.
    // all lines are of the same width, extra space is distributed evenly).
}
