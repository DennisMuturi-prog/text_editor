// Source - https://stackoverflow.com/a
// Posted by Kevin Reid, modified by community. See post 'Timeline' for change history
// Retrieved 2026-01-06, License - CC BY-SA 4.0

use std::ops::{Deref, Range};
use std::rc::Rc;

use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RcSubstr {
    string: Rc<str>,
    span: Range<usize>,
    boundaries:Rc<[usize]>
}

impl RcSubstr {
    pub fn new(string: Rc<str>) -> Self {
        let span = 0..string.graphemes(true).count();
        let boundaries=find_grapheme_boundaries(&string);
        Self { string, span,boundaries }
    }
    pub fn substr(&self, span: Range<usize>) -> Self {
        // A full implementation would also have bounds checks to ensure
        // the requested range is not larger than the current substring
        Self {
            string: Rc::clone(&self.string),
            span: (self.span.start + span.start)..(self.span.start + span.end),
            boundaries: Rc::clone(&self.boundaries)
        }
    }
    pub fn get_part_of_string(&self,span: Range<usize>)->&str{
        &self.string[self.boundaries[self.span.start+span.start]..self.boundaries[self.span.start+span.end]]
        
        
    }
}

impl Deref for RcSubstr {
    type Target = str;
    fn deref(&self) -> &str {
        // println!("span is {:?} and str is {}",self.boundaries,self.string);
        &self.string[self.boundaries[self.span.start]..self.boundaries[self.span.end]]
    }
}
pub fn find_grapheme_boundaries(content: &Rc<str>) -> Rc<[usize]> {
    use unicode_segmentation::UnicodeSegmentation;
    
    let mut boundaries = Vec::new();
    let mut byte_offset = 0;
    
    // Iterate over grapheme clusters with extended grapheme clusters
    for grapheme in content.graphemes(true) {
        boundaries.push(byte_offset);
        byte_offset += grapheme.len();
    }
    boundaries.push(byte_offset);
   
   // Add the end boundary
    boundaries.into()
}



// fn main() {
//     let s = R);
//     let u = s.substr(1..2);

//     // We need to deref to print the string rather than the wrapper struct.
//     // A full implementation would `impl Debug` and `impl Display` to produce
//     // the expected substring.
//     println!("{}", &*u);
// }
