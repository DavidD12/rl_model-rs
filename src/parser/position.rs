use line_col::LineColLookup;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(lookup: &LineColLookup, offset: usize) -> Self {
        let (line, column) = lookup.get(offset);
        Self { line, column }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
