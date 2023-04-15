pub struct StrSplit<'a> {
    // what havent we looked at
    remainder: &'a str,
    delimiter: &'a str,
}


// you can only keep using the StrSplit as long as the
// input strings are not de allocated
impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: haystack,
            delimiter,
        }
    }
}

//iterate over StrSplit
impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

    // find the next delimiter if one exists and chop off the rest
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            let until_delimiter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else if self.remainder.is_empty() {
            None
        } else {
            let rest = self.remainder;
            self.remainder = &[];
            Some(rest)
        }
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";

    let letters = StrSplit::new(haystack, " ");

    assert_eq!(letters, vec!["a", "b", "c", "d", "e"].into_iter())
}
