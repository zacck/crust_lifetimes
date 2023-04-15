//Takes a string and splits into smaller
// Strings separated by a delimiter
#[derive(Debug)]
pub struct StrSplit<'a> {
    // what havent we looked at
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

// you can only keep using the StrSplit as long as the
// input strings are not de allocated
impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

//iterate over StrSplit
impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

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
