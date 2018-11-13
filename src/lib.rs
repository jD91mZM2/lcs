use std::cmp;

/// The preferred ordering in cases where two paths are equally long
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Ordering {
    DeleteFirst,
    InsertFirst
}

/// A diff component
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Diff<'a, T: 'a + Eq> {
    Common(&'a T),
    Delete(&'a T),
    Insert(&'a T)
}

/// Data needed for doing LCS operations on two slices
#[derive(Clone)]
pub struct Lcs<'a, T: 'a + Eq> {
    source: &'a [T],
    dest: &'a [T],
    matrix: Box<[Box<[usize]>]>
}
impl<'a, T: 'a + Eq> Lcs<'a, T> {
    /// Pre-calculate necessary stuff and return an instance ready to be used
    pub fn new(source: &'a [T], dest: &'a [T]) -> Self {
        // ALGORITHM: https://en.wikipedia.org/wiki/Longest_common_subsequence_problem#Worked_example
        // It works by computing a sort of table that counts equal (and in
        // order) elements, or defaults to taking the longest one after
        // deleting from either the source or destination.
        // Example:
        //    0[H]e y[!]
        //  0 0 0 0 0 0
        // [H]0[1]1 1 1
        //  i 0 1 1 1 1
        // [!]0 1 1 1[2]

        let mut matrix = vec![vec![0; 1 + source.len()].into_boxed_slice(); 1 + dest.len()].into_boxed_slice();
        for y in 1..=dest.len() {
            for x in 1..=source.len() {
                if source[x-1] == dest[y-1] {
                    matrix[y][x] = matrix[y-1][x-1] + 1;
                } else {
                    matrix[y][x] = cmp::max(matrix[y][x-1], matrix[y-1][x]);
                }
            }
        }
        Lcs {
            source,
            dest,
            matrix
        }
    }
    /// Get the total length of all the longest possible subsequences. Getting
    /// the length of the LCS of "Hi!" and "Hey!" returns 2, because that's the
    /// number of common characters ('H' and '!').
    pub fn length(&self) -> usize {
        self.matrix[self.dest.len()][self.source.len()]
    }
    /// Backtrack a diff out of the LCS. In cases where there are multiple
    /// equal paths, it chooses the one which the specified ordering prefers.
    /// "abc" to "acb" could either be "a -b c +b" (deletion first) or "a +c b
    /// -c" (insertion first).
    pub fn backtrack(&self, ordering: Ordering) -> Vec<Diff<T>> {
        // Backtracks a diff using the matrix. It walks through the path back,
        // choosing the path with longest length. If two paths have the same
        // length, it chooses the one which results in diff items coming in the
        // specified preferred ordering.
        // Example (deletion first):
        //    0[H]e y[!]
        //  0[0]0 0 0 0
        // [H]0[1]1 1 1
        //  i 0[1|1|1]1
        // [!]0 1 1 1[2]
        let mut track = Vec::new();

        let mut y = self.dest.len();
        let mut x = self.source.len();
        while y > 0 || x > 0 {
            if x > 0 && y > 0 && self.source[x-1] == self.dest[y-1] {
                x -= 1;
                y -= 1;
                track.push(Diff::Common(&self.source[x]));
            } else if x == 0 || self.matrix[y-1][x] > self.matrix[y][x-1]
                    // If ordering is delete first we actually want to handle
                    // insertions first since we're backtracking!
                    || (ordering == Ordering::DeleteFirst && self.matrix[y-1][x] == self.matrix[y][x-1]) {
                y -= 1;
                track.push(Diff::Insert(&self.dest[y]))
            } else {
                x -= 1;
                track.push(Diff::Delete(&self.source[x]))
            }
        }

        track.reverse();
        track
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length() {
        assert_eq!(Lcs::new(b"Hi!", b"Hey!").length(), 2);
        assert_eq!(Lcs::new(b"Hello!", b"Hello :D!").length(), 6);
    }
    #[test]
    fn diff() {
        // abc -> acb (ordering matters significantly)
        assert_eq!(
            Lcs::new(b"abc", b"acb").backtrack(Ordering::DeleteFirst),
            vec![Diff::Common(&b'a'), Diff::Delete(&b'b'), Diff::Common(&b'c'), Diff::Insert(&b'b')]
        );
        assert_eq!(
            Lcs::new(b"abc", b"acb").backtrack(Ordering::InsertFirst),
            vec![Diff::Common(&b'a'), Diff::Insert(&b'c'), Diff::Common(&b'b'), Diff::Delete(&b'c')]
        );

        // abc -> acd (ordering doesn't matter since it doesn't have a choice)
        assert_eq!(
            Lcs::new(b"abc", b"acd").backtrack(Ordering::DeleteFirst),
            vec![Diff::Common(&b'a'), Diff::Delete(&b'b'), Diff::Common(&b'c'), Diff::Insert(&b'd')]
        );
        assert_eq!(
            Lcs::new(b"abc", b"acd").backtrack(Ordering::InsertFirst),
            vec![Diff::Common(&b'a'), Diff::Delete(&b'b'), Diff::Common(&b'c'), Diff::Insert(&b'd')]
        );

        // Hi! -> Hey! (ordering matters slightly)
        assert_eq!(
            Lcs::new(b"Hi!", b"Hey!").backtrack(Ordering::DeleteFirst),
            vec![Diff::Common(&b'H'), Diff::Delete(&b'i'), Diff::Insert(&b'e'), Diff::Insert(&b'y'), Diff::Common(&b'!')]
        );
        assert_eq!(
            Lcs::new(b"Hi!", b"Hey!").backtrack(Ordering::InsertFirst),
            vec![Diff::Common(&b'H'), Diff::Insert(&b'e'), Diff::Insert(&b'y'), Diff::Delete(&b'i'), Diff::Common(&b'!')]
        );
    }
}
