use chrono::Utc;

#[derive(Debug, PartialEq)]
pub(crate) enum State {
    Follower,
    Candidate,
    Leader,
}

#[derive(Debug)]
pub (crate) enum NodeMessage {
    Heartbeat { term: u64, leader_id: u64 },

    RequestVote { term: u64, candidate_id: u64 },
    RejectVote { term: u64, candidate_id: u64 },
    GrantVote { term: u64, candidate_id: u64 },
}

pub (crate) struct Node {
    id: u64,
    state: State,
    current_term: u64,

    next_heartbeat_timeout: chrono::Duration,
    last_heartbeat: chrono::DateTime<Utc>,
}
