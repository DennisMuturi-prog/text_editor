// Source - https://stackoverflow.com/a
// Posted by Kevin Reid, modified by community. See post 'Timeline' for change history
// Retrieved 2026-01-06, License - CC BY-SA 4.0

use std::ops::{Deref, Range};
use std::rc::Rc;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RcSubstr {
    string: Rc<str>,
    span: Range<usize>,
}

impl RcSubstr {
    fn new(string: Rc<str>) -> Self {
        let span = 0..string.chars().count();
        Self { string, span }
    }
    fn substr(&self, span: Range<usize>) -> Self {
        // A full implementation would also have bounds checks to ensure
        // the requested range is not larger than the current substring
        Self {
            string: Rc::clone(&self.string),
            span: (self.span.start + span.start)..(self.span.start + span.end)
        }
    }
}

impl Deref for RcSubstr {
    type Target = str;
    fn deref(&self) -> &str {
        &self.string[self.span.clone()]
    }
}

fn main() {
    let s = RcSubstr::new(Rc::<str>::from("foo"));
    let u = s.substr(1..2);
    
    // We need to deref to print the string rather than the wrapper struct.
    // A full implementation would `impl Debug` and `impl Display` to produce
    // the expected substring.
    println!("{}", &*u);
}
