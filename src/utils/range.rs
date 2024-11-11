#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Range {
    pub start: i64,
    pub end: i64,  // inclusive
}

impl Range {
    pub fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }

    pub fn contains(&self, point: i64) -> bool {
        self.start <= point && point <= self.end
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    pub fn merge(&self, other: &Range) -> Option<Range> {
        if self.overlaps(other) {
            Some(Range::new(
                self.start.min(other.start),
                self.end.max(other.end)
            ))
        } else {
            None
        }
    }

    pub fn length(&self) -> i64 {
        self.end - self.start + 1
    }
} 