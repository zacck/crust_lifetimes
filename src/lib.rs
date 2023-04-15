pub struct StrSplit {}

impl StrSplit {
    pub fn new(haystack: &str, delimiter: &str) -> Self {}
}


//iterate over StrSplit
impl Iterator for StrSplit {
    type Item = &str;

    fn next (&mut self) -> Option<Self::Item> {

    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";

    let letters =  StrSplit::new(haystack, " ");

    assert_eq!(letters, vec!["a", "b", "c", "d", "e"].into_iter())
}
