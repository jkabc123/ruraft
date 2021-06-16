use crate::LogEntry;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RaftMessage<T>
where
    T: Sized + Clone + PartialEq + Eq + Debug + Default,
{
    ClientRequest {
        dest: usize,
        value: T,
    },
    BecomeLeader {
        dest: usize,
        followers: Vec<usize>,
    },
    AppendEntries {
        dest: usize,
        followers: Vec<usize>,
    },
    AppendEntriesRequest {
        src: usize,
        dest: usize,
        term: usize,
        prev_index: usize,
        prev_term: usize,
        entries: Vec<LogEntry<T>>,
    },
    AppendEntriesResponse {
        src: usize,
        dest: usize,
        term: usize,
        success: bool,
        match_index: usize,
    },
}