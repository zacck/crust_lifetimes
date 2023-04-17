//Takes a string and splits into smaller
// Strings separated by a delimiter
#[derive(Debug)]
pub struct StrSplit<'haystack, 'delimiter> {
    // what havent we looked at
    remainder: Option<&'haystack str>,
    delimiter: &'delimiter str,
}

// you can only keep using the StrSplit as long as the
// input strings are not de allocated
impl<'haystack, 'delimiter> StrSplit<'haystack, 'delimiter> {
    pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

//iterate over StrSplit
impl<'haystack, 'delimiter> Iterator for StrSplit<'haystack, 'delimiter> {
    type Item = &'haystack str;

    // find the next delimiter if one exists and chop off the rest
    fn next(&mut self) -> Option<Self::Item> {
        //get a mutable reference to what is in remainder
        if let Some(ref mut remainder) = self.remainder {
            if let Some(next_delim) = remainder.find(self.delimiter) {
                let until_delimiter = &remainder[..next_delim];
                *remainder = &remainder[(next_delim + self.delimiter.len())..];
                Some(until_delimiter)
            } else {
                //if remainder search the remainder
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

//we can use StrSplit here since we should
//stop when the delimiter first appears we only
//do one step of the iteration and return the collected slice
fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, &format!("{}", c))
        .next()
        .expect("StrSplit always gives at least one result")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello_world", 'o'), "hell");
}

#[test]
fn it_works() {
    let haystack = "a b c d e";

    let letters = StrSplit::new(haystack, " ");

    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
}

#[test]
fn collects_empty_tail() {
    let haystack = "a b c d ";

    let letters = StrSplit::new(haystack, " ");

    assert!(letters.eq(vec!["a", "b", "c", "d", ""].into_iter()));
}
