pub trait TextRepresentation {
    fn insert(&mut self, content: String, index: usize) -> usize;
    fn delete(&mut self, length_to_cut: usize, index: usize) -> usize;
    fn undo(&mut self) -> Option<usize>;
    fn redo(&mut self) -> Option<usize>;
    fn collect_string(&self, text: &mut String);
    fn collect_substring(&self, text: &mut String, bounds: (usize, usize));
}
