#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub requested_funds: u64,
    pub allocated: bool,
}

#[contracttype]
pub enum FundBook {
    Proposal(u64),
}

const FUND_COUNT: Symbol = symbol_short!("F_COUNT");

#[contract]
pub struct CommunityFundAllocation;

#[contractimpl]
impl CommunityFundAllocation {
    pub fn create_proposal(env: Env, title: String, description: String, requested_funds: u64) -> u64 {
        let mut count = env.storage().instance().get(&FUND_COUNT).unwrap_or(0);
        count += 1;

        let proposal = Proposal {
            id: count,
            title,
            description,
            requested_funds,
            allocated: false,
        };

        env.storage().instance().set(&FundBook::Proposal(count), &proposal);
        env.storage().instance().set(&FUND_COUNT, &count);

        count
    }

    pub fn allocate_funds(env: Env, proposal_id: u64) {
        let key = FundBook::Proposal(proposal_id);
        let mut proposal: Proposal = env.storage().instance().get(&key).expect("Proposal not found");

        if proposal.allocated {
            panic!("Funds already allocated!");
        }

        proposal.allocated = true;
        env.storage().instance().set(&key, &proposal);
    }

    pub fn get_proposal(env: Env, proposal_id: u64) -> Proposal {
        env.storage().instance().get(&FundBook::Proposal(proposal_id)).unwrap_or(Proposal {
            id: 0,
            title: String::from_str(&env, "Not Found"),
            description: String::from_str(&env, "Not Found"),
            requested_funds: 0,
            allocated: false,
        })
    }

    pub fn total_proposals(env: Env) -> u64 {
        env.storage().instance().get(&FUND_COUNT).unwrap_or(0)
    }
}
