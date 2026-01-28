use soroban_sdk::{contracttype, Address, BytesN, Symbol, symbol_short};

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum ProposalStatus {
    Pending,
    Active,
    Approved,
    Rejected,
    Executed,
    Expired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum VoteType {
    For,
    Against,
    Abstain,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum VotingScheme {
    OnePersonOneVote,
    TokenWeighted,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct Proposal {
    pub id: u32,
    pub proposer: Address,
    pub new_wasm_hash: BytesN<32>,
    pub description: Symbol,
    pub created_at: u64,
    pub voting_start: u64,
    pub voting_end: u64,
    pub execution_delay: u64,
    pub status: ProposalStatus,
    pub votes_for: i128,
    pub votes_against: i128,
    pub votes_abstain: i128,
    pub total_votes: u32,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct GovernanceConfig {
    pub voting_period: u64,
    pub execution_delay: u64,
    pub quorum_percentage: u32,  // Basis points (e.g., 5000 = 50%)
    pub approval_threshold: u32,  // Basis points (e.g., 6667 = 66.67%)
    pub min_proposal_stake: i128,
    pub voting_scheme: VotingScheme,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct Vote {
    pub voter: Address,
    pub proposal_id: u32,
    pub vote_type: VoteType,
    pub voting_power: i128,
    pub timestamp: u64,
}

// Storage keys
pub const PROPOSALS: Symbol = symbol_short!("PROPOSALS");
pub const PROPOSAL_COUNT: Symbol = symbol_short!("PROP_CNT");
pub const VOTES: Symbol = symbol_short!("VOTES");
pub const GOVERNANCE_CONFIG: Symbol = symbol_short!("GOV_CFG");
pub const VOTER_REGISTRY: Symbol = symbol_short!("VOTERS");
