// !
// uncomment when finished writing code
// #![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

// Lifetimes
// - How long can a pointer be used for (e.g, until memory is deallocated)
// - <'a> means how long `this` reference lives for
// - Anonymous lifetimes (<'_>) tells the compiler to guess the lifetime & only work when there is 1 possible guess
// - <'static> lives for the entire duration of the program
// - only need to use double lifetimes when using multiple references & is important they aren't the same bc you want to return one w/o tying it to the other

#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>, // part of the string we haven't looked at
    delimiter: D,                      // what are we splitting by
}

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

pub trait Delimiter {
    // returns where it starts + ends
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter,
{
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        // takes mutable reference to self + returns Option<&mut T>
        let remainder = self.remainder.as_mut()?;

        if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
            // if next_delim does appear in str
            let until_delimiter = &remainder[..delim_start];

            // reassign remainder to be everything past delimiter
            *remainder = &remainder[delim_end..];

            Some(until_delimiter)
        } else {
            // return the remainder that doesn't have a delimiter once
            // take so we leave none in it's place
            self.remainder.take()
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        // find : gives option of the position found
        // map : if None, return None. If some, change contents
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        // iterating over all characters of string
        s.char_indices()
            // looking for the one we're searching for
            .find(|(_, c)| c == self)
            // when find, map Some() to map that position + utf8 length (e.g, 1 for letter)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

// Gives string until first occurrence of character (c)
pub fn until_char(s: &str, c: char) -> &'_ str {
    StrSplit::new(s, c)
        .next()
        .expect("StrSplit always gives at least one result")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell");
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn tail() {
    let haystack = "a b c d";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}
