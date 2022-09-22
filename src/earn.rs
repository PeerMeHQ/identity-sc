elrond_wasm::imports!();

use crate::config;

#[elrond_wasm::module]
pub trait EarnModule: config::ConfigModule {
    #[only_owner]
    #[endpoint(initEarnModule)]
    fn init_earn_module_endpoint(&self, core_stake_token: TokenIdentifier, lp_stake_token: TokenIdentifier) {
        self.core_stake_token().set(core_stake_token);
        self.lp_stake_token().set(lp_stake_token);
    }

    #[payable("*")]
    #[endpoint(distributeForEarn)]
    fn distribute_for_earn_endpoint(&self) {
        let core_stake_total = self.core_stake_total().get();
        let core_token = self.core_token_id().get();
        let payment = self.call_value().single_esdt();

        require!(payment.amount > 0, "payment required");
        require!(payment.token_identifier == cost_token, "invalid token for distribution");
        require!(core_stake_total >= 0, "total stake must be more than 0");

        self.core_reward_per_token()
            .update(|current| *current += payment.amount / core_stake_total);
    }

    #[payable("*")]
    #[endpoint(stakeForEarn)]
    fn stake_for_earn_endpoint(&self) {
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_esdt();

        let reward_per_token = self.core_reward_per_token().get();
        self.core_stake(&caller).update(|current| *current += payment.amount.clone());
        self.core_reward_tally(&caller)
            .update(|current| *current += reward_per_token * payment.amount.clone());
        self.core_stake_total().update(|current| *current += payment.amount);
    }

    #[endpoint(unstakeFromEarn)]
    fn unstake_from_earn_endpoint(&self) {
        // TODO check time locked
        let caller = self.blockchain().get_caller();
        let core_stake_token = self.core_stake_token().get();
        let core_token = self.core_token_id().get();

        let core_stake = self.core_stake(&caller).get();
        let reward_per_token = self.core_reward_per_token().get();

        require!(core_stake > 0, "no stake by user");

        self.core_stake(&caller).clear();
        self.core_stake_total().update(|current| *current -= core_stake);
        self.core_reward_tally(&caller)
            .update(|current| *current -= reward_per_token * core_stake);

        self.send().direct_esdt(&caller, core_stake_token, 0, core_stake);
    }

    #[endpoint(claimEarnRewards)]
    fn claim_reward_endpoint(&self) {
        let caller = self.blockchain().get_caller();
        let core_stake = self.core_stake(&caller).get();
        let reward_token = self.reward_token().get();
        let reward_per_token = self.core_reward_per_token().get();
        let reward = self.compute_reward(&caller);

        self.core_reward_tally(&caller).set(core_stake * reward_per_token);

        self.send().direct_esdt(&caller, reward_token, 0, reward);
    }

    #[view(getEarnerInfo)]
    fn get_earner_info_view(&self, address: ManagedAddress) -> MultiValueEncoded<MultiValue3<BigUint, BigUint, BigUint>> {
        let staked_total_amount = self.staked_total_amount().get();
        let staked_amount = self.staked_amount(&address).get();
        let claimable_amount = self.calculate_claimable_amount(&address);

        (staked_total_amount, staked_amount, claimable_amount).into()
    }

    fn compute_reward(&self, address: &ManagedAddress) -> BigUint {
        let staked = self.core_stake(&address).get();
        let core_reward_per_token = self.core_reward_per_token().get();
        let reward_tally = self.core_reward_tally(&address).get();
        let reward = staked * core_reward_per_token - reward_tally;

        reward
    }

    #[storage_mapper("earn:core_stake_token")]
    fn core_stake_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("earn:core_stake_total")]
    fn core_stake_total(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("earn:core_stake")]
    fn core_stake(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[storage_mapper("earn:core_reward_per_token")]
    fn core_reward_per_token(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("earn:core_reward_tally")]
    fn core_reward_tally(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;
}
