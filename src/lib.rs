/// Assumes the first word of the line is already at the start of the line,
/// without spaces following it.
///
/// # Examples
///
/// ```
/// use justify_string::finalize_current_line;
///
/// let mut out = String::from("123");
/// finalize_current_line(&mut out, &mut vec!["12", "1234"], 3);
/// assert_eq!(out, String::from("123   12  1234"));
/// ```
///
/// If `words_after_first` is empty, it simply appends extra spaces to the first word.
///
/// ```
/// use justify_string::finalize_current_line;
///
/// let mut out = String::from("123");
/// finalize_current_line(&mut out, &mut vec![], 2);
/// assert_eq!(out, String::from("123  "));
/// ```
pub fn finalize_current_line(
    output: &mut String,
    words_after_first: &Vec::<&str>,
    num_extra_spaces: u32,
) {
    let append_spaces = |s: &mut String, num_spaces: u32| {
        for _ in 0..num_spaces {
            s.push(' ');
        }
    };
    let num_words_after_first = words_after_first.len() as u32;

    if num_words_after_first <= 0 {
        append_spaces(output, num_extra_spaces);
        return;
    }

    // Distribute extra spaces evenly between gaps.
    let num_gaps_between_words = num_words_after_first;
    // `+ 1` because it's _extra_ spaces, at least one is necessary.
    let small_gap_size = 1 + num_extra_spaces / num_gaps_between_words;
    let big_gap_size = small_gap_size + 1;
    // This is guaranteed to be smaller than `words_after_first.len()`.
    let num_big_gaps = num_extra_spaces % num_gaps_between_words;
    let mut words = words_after_first.iter();
    let mut append_spaces_and_word = |num_spaces: u32, word: &str| {
        append_spaces(output, num_spaces);
        output.push_str(word);
    };
    // TODO perf: do `unwrap_unchecked`? Or can slices help somehow?
    for _ in 0..num_big_gaps {
        // It's ok to `unwrap` because `num_big_gaps` is guaranteed to be smaller than
        // `words_after_first.len()`
        append_spaces_and_word(big_gap_size, words.next().unwrap());
    }
    // Now small gaps.
    // TODO perf: It's guaranteed that there's at least one element left, so `unwrap_unchecked`?
    for word in words {
        append_spaces_and_word(small_gap_size, word);
    }
}

/**
# Examples

```
use justify_string::justify;

assert_eq!(
    justify("123 12 123456789abc", 8),
    concat!(
        "123   12", "\n",
        "12345678", "\n",
        "9abc    ",
    )
);
```

See tests for more.

# Panics

If `line_width < 1`
*/
pub fn justify(input: &str, line_width: u32) -> String {
    // TODO consider making the `line_width` parameter of type `NonZeroU32`.
    assert!(line_width >= 1);

    // TODO perf: do we need optimizations in case `line_width >= input.chars()len()`?
    // In case `line_width <= 2`? In case input words are guaranteed to be separated by only a
    // single space?
    //  Maybe with an argument, or a config var.

    // TODO how about `split_ascii_whitespace`?
    let mut words = input.split_whitespace();
    // TODO perf: does the program actually search for the end of the word
    // when this statement gets executed? If so, it's inefficient - if we're
    // starting a new line, we can start copying chars of the next word
    // right away, without trying to first find where it ends.
    // Same for the next `words.next()` below.
    let mut first_word_of_line = match words.next() {
        None => return "".to_string(),
        Some(w) => w,
    };
    // The resulting string is most likely gonna be a little longer than the original one.
    // But it may also be shorter if there is a lot of spaces.
    // TODO perf: maybe allocate a little more than this?
    let mut res = String::with_capacity(input.len());
    // TODO perf: how about we instead use an array on the stack? This would put a cap
    // on `line_width` though.
    // TODO perf: if not, maybe don't allocate the max theoretical capacity?
    // Also, it's guaranteed that this capacity won't even be exceeded, so `push` doesn't
    // have to check if it needs to be expanded.
    let mut words_after_first = {
        // TODO refactor: add tests to check if this is calculated correctly?
        let max_words_per_line = (line_width + 1) / 2;
        Vec::<&str>::with_capacity(max_words_per_line as usize - 1)
    };
    /// If the word fits, appends it to `words_after_first` and returns `Ok()`,
    /// if it doesn't, returns `Error` with that word.
    fn try_append_word_to_line<'a>(
        words_after_first: &mut Vec<&'a str>,
        word: &'a str,
        line_remaining_capacity_chars: &mut u32,
    ) -> Result<(), &'a str> {
        // `+ 1` because there needs to be a space before this word.
        let requred_capacity_to_append_curr_word = word.chars().count() as u32 + 1;
        if requred_capacity_to_append_curr_word <= *line_remaining_capacity_chars {
            words_after_first.push(word);
            *line_remaining_capacity_chars -= requred_capacity_to_append_curr_word;
            Ok(())
        } else {
            Err(word)
        }
    }

    loop {
        // New line, new word.
        let mut line_remaining_capacity_chars = line_width;

        // Put the first word at the beginning of the line.
        // If `line_width` is exceeded, add a line break.
        // TODO perf: maybe it's guaranteed by the caller that it's not possible for any word
        // to be longer than `line_width`? Add a parameter or a config var.
        // TODO perf: Each word consists of at least one char, so no need to check if the first one
        // is `Some`.
        for ch in first_word_of_line.chars() {
            // TODO perf: does the compiled code perform this check on the first iteration?
            // Could adding an assert that `line_width > 0` help?
            if line_remaining_capacity_chars <= 0 {
                res.push('\n');
                line_remaining_capacity_chars = line_width;
            }

            res.push(ch);
            line_remaining_capacity_chars -= 1;
        }

        // The first word of the line is put, now the following ones.
        first_word_of_line = 'words_after_first_of_line: loop {
            let word = match words.next() {
                None => {
                    finalize_current_line(&mut res, &words_after_first, line_remaining_capacity_chars);
                    return res;
                },
                Some(w) => w,
            };

            if let Err(failed_to_append) = try_append_word_to_line(
                &mut words_after_first,
                word,
                &mut line_remaining_capacity_chars
            ) {
                // We'll put this word on a new line.
                break 'words_after_first_of_line failed_to_append;
            }
        };

        finalize_current_line(&mut res, &words_after_first, line_remaining_capacity_chars);
        // TODO refactor: Is there a better way? Can we just declare it for each line
        // without reallocating?
        words_after_first.clear();
        res.push('\n');
    }
}

#[cfg(test)]
mod tests {
    use super::justify;

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

            (lorem_ipsum, 12, concat!(
             "Lorem  ipsum\n",
             "dolor    sit\n",
             "amet        \n",
             "consectetur \n",
             "adipiscing  \n",
             "elit  sed do\n",
             "eiusmod     \n",
             "tempor      \n",
             "incididunt  \n",
             "ut labore et\n",
             "dolore magna\n",
             "aliqua      ")),
            (lorem_ipsum, 7, concat!(
             "Lorem  \n",
             "ipsum  \n",
             "dolor  \n",
             "sit    \n",
             "amet   \n",
             "consect\n",
             "etur   \n",
             "adipisc\n",
             "ing    \n",
             "elit   \n",
             "sed  do\n",
             "eiusmod\n",
             "tempor \n",
             "incidid\n",
             "unt  ut\n",
             "labore \n",
             "et     \n",
             "dolore \n",
             "magna  \n",
             "aliqua ")),
            (lorem_ipsum, 35, concat!(
             "Lorem    ipsum   dolor   sit   amet\n",
             "consectetur  adipiscing elit sed do\n",
             "eiusmod tempor incididunt ut labore\n",
             "et      dolore     magna     aliqua")),
        ];

        for &(input, line_width, expected) in &test_cases {
            assert_eq!(
                justify(input, line_width), expected,
                "input: \"{}\", width: {}", input, line_width
            );
        }
    }
    // TODO refactor: add randomized tests with algorithmic correctness checks (i.e.
    // all lines are of the same width, extra space is distributed evenly).
    // TODO refactor: use tests from https://github.com/ctrlcctrlv/justify/blob/master/tests/tests.rs
}
