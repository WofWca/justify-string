pub fn transform(input: &str, line_width: u32) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::transform;

    #[test]
    fn simple() {
        let test_cases = [
            ("", 5, ""),
            ("test", 5, "test "),
            ("Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua", 12,
             "Lorem  ipsum\ndolor    sit\namet        \nconsectetur \nadipiscing  \nelit  sed do\neiusmod     \ntempor      \nincididunt  \nut labore et\ndolore magna\naliqua      ")
        ];

        for &(input, line_width, expected) in &test_cases {
            assert_eq!(
                transform(input, line_width), expected,
                "input: \"{}\", width: {}", input, line_width
            );
        }
    }
}
