//Takes a string and splits into smaller
// Strings separated by a delimiter
#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    // what havent we looked at
    remainder: Option<&'haystack str>,
    delimiter: D,
}

// you can only keep using the StrSplit as long as the
// input strings are not de allocated
impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

//Anything that can find itself in a string
pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

//iterate over StrSplit
impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter,
{
    type Item = &'haystack str;

    // find the next delimiter if one exists and chop off the rest
    fn next(&mut self) -> Option<Self::Item> {
        //get a mutable reference to what is in remainder
        if let Some(ref mut remainder) = self.remainder {
            if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
                let until_delimiter = &remainder[..delim_start];
                *remainder = &remainder[delim_end..];
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

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + 1))
    }
}

//we can use StrSplit here since we should
//stop when the delimiter first appears we only
//do one step of the iteration and return the collected slice
fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, c)
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
