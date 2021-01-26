use casper_execution_engine::{
    core::engine_state::{
        step::{RewardItem, SlashItem},
        StepRequest,
    },
    shared::newtypes::Blake2bHash,
};
use casper_types::ProtocolVersion;

#[derive(Debug)]
pub struct StepRequestBuilder {
    parent_state_hash: Blake2bHash,
    protocol_version: ProtocolVersion,
    slash_items: Vec<SlashItem>,
    reward_items: Vec<RewardItem>,
    run_auction: bool,
    next_era_id: u64,
}

impl StepRequestBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_parent_state_hash(mut self, parent_state_hash: Blake2bHash) -> Self {
        self.parent_state_hash = parent_state_hash;
        self
    }

    pub fn with_protocol_version(mut self, protocol_version: ProtocolVersion) -> Self {
        self.protocol_version = protocol_version;
        self
    }

    pub fn with_slash_item(mut self, slash_item: SlashItem) -> Self {
        self.slash_items.push(slash_item);
        self
    }

    pub fn with_reward_item(mut self, reward_item: RewardItem) -> Self {
        self.reward_items.push(reward_item);
        self
    }

    pub fn with_run_auction(mut self, run_auction: bool) -> Self {
        self.run_auction = run_auction;
        self
    }

    pub fn with_next_era_id(mut self, next_era_id: u64) -> Self {
        self.next_era_id = next_era_id;
        self
    }

    pub fn build(self) -> StepRequest {
        StepRequest::new(
            self.parent_state_hash,
            self.protocol_version,
            self.slash_items,
            self.reward_items,
            self.run_auction,
            self.next_era_id,
        )
    }
}

impl Default for StepRequestBuilder {
    fn default() -> Self {
        StepRequestBuilder {
            parent_state_hash: Default::default(),
            protocol_version: Default::default(),
            slash_items: Default::default(),
            reward_items: Default::default(),
            next_era_id: Default::default(),
            run_auction: true, //<-- run_auction by default
        }
    }
}
