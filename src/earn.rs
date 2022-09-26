elrond_wasm::imports!();

use crate::config;

const PRECISION: u32 = 18;

#[elrond_wasm::module]
pub trait EarnModule: config::ConfigModule {
    #[only_owner]
    #[endpoint(initEarnModule)]
    fn init_earn_module_endpoint(&self, core_stake_token: TokenIdentifier, lp_stake_token: TokenIdentifier) {
        self.core_stake_token().set(core_stake_token);
        self.lp_stake_token().set(lp_stake_token);
    }

    #[payable("*")]
    #[endpoint(distributeForCoreEarn)]
    fn distribute_for_core_earn_endpoint(&self) {
        let payment = self.call_value().single_esdt();
        let core_token = self.core_token().get();
        let core_stake_total = self.core_stake_total().get();

        require!(payment.amount > 0, "invalid amount");
        require!(payment.token_identifier == core_token, "invalid token");
        require!(core_stake_total >= 0, "total stake must be more than 0");

        self.core_reward_per_token()
            .update(|current| *current += payment.amount.clone() * BigUint::from(10u64).pow(PRECISION) / core_stake_total);
    }

    #[payable("*")]
    #[endpoint(distributeForLpEarn)]
    fn distribute_for_lp_earn_endpoint(&self) {
        let payment = self.call_value().single_esdt();
        let core_token = self.core_token().get();
        let lp_stake_total = self.lp_stake_total().get();

        require!(payment.amount > 0, "invalid amount");
        require!(payment.token_identifier == core_token, "invalid token");
        require!(lp_stake_total >= 0, "total stake must be more than 0");

        self.lp_reward_per_token()
            .update(|current| *current += payment.amount.clone() * BigUint::from(10u64).pow(PRECISION) / lp_stake_total);
    }

    #[payable("*")]
    #[endpoint(stakeForEarn)]
    fn stake_for_earn_endpoint(&self) {
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_esdt();

        if payment.token_identifier == self.core_stake_token().get() {
            let rpt = self.core_reward_per_token().get();
            self.core_stake(&caller).update(|current| *current += payment.amount.clone());
            self.core_reward_tally(&caller).update(|current| *current += rpt * payment.amount.clone());
            self.core_stake_total().update(|current| *current += payment.amount);
        } else if payment.token_identifier == self.lp_stake_token().get() {
            let rpt = self.lp_reward_per_token().get();
            self.lp_stake(&caller).update(|current| *current += payment.amount.clone());
            self.lp_reward_tally(&caller).update(|current| *current += rpt * payment.amount.clone());
            self.lp_stake_total().update(|current| *current += payment.amount);
        } else {
            sc_panic!("invalid stake token");
        }
    }

    #[endpoint(unstakeFromEarn)]
    fn unstake_from_earn_endpoint(&self, token: TokenIdentifier) {
        let caller = self.blockchain().get_caller();

        // TODO check time locked

        if token == self.core_stake_token().get() {
            let core_stake = self.core_stake(&caller).get();
            let core_rpt = self.core_reward_per_token().get();
            require!(core_stake > 0, "no stake by user");
            self.core_stake(&caller).clear();
            self.core_stake_total().update(|current| *current -= core_stake.clone());
            self.core_reward_tally(&caller).update(|current| *current -= core_rpt * core_stake.clone());
            self.send().direct_esdt(&caller, &token, 0, &core_stake);
        } else if token == self.lp_stake_token().get() {
            let lp_stake = self.lp_stake(&caller).get();
            let lp_rpt = self.lp_reward_per_token().get();
            require!(lp_stake > 0, "no stake by user");
            self.lp_stake(&caller).clear();
            self.lp_stake_total().update(|current| *current -= lp_stake.clone());
            self.lp_reward_tally(&caller).update(|current| *current -= lp_rpt * lp_stake.clone());
            self.send().direct_esdt(&caller, &token, 0, &lp_stake);
        } else {
            sc_panic!("invalid token");
        }
    }

    #[endpoint(claimEarnRewards)]
    fn claim_earn_reward_endpoint(&self) {
        let caller = self.blockchain().get_caller();
        let reward_total = self.compute_reward(&caller);

        let core_stake = self.core_stake(&caller).get();
        let core_rpt = self.core_reward_per_token().get();
        self.core_reward_tally(&caller).set(core_stake * core_rpt);

        let lp_stake = self.lp_stake(&caller).get();
        let lp_rpt = self.lp_reward_per_token().get();
        self.lp_reward_tally(&caller).set(lp_stake * lp_rpt);

        let core_token = self.core_token().get();
        self.send().direct_esdt(&caller, &core_token, 0, &reward_total);
    }

    #[view(getEarnerInfo)]
    fn get_earner_info_view(&self, address: ManagedAddress) -> MultiValue3<BigUint, BigUint, BigUint> {
        let core_stake = self.core_stake(&address).get();
        let lp_stake = self.lp_stake(&address).get();
        let reward_total = self.compute_reward(&address);

        (core_stake, lp_stake, reward_total).into()
    }

    fn compute_reward(&self, address: &ManagedAddress) -> BigUint {
        let core_staked = self.core_stake(&address).get();
        let core_reward_per_token = self.core_reward_per_token().get();
        let core_reward_tally = self.core_reward_tally(&address).get();
        let core_reward = core_staked * core_reward_per_token - core_reward_tally;

        let lp_staked = self.lp_stake(&address).get();
        let lp_reward_per_token = self.lp_reward_per_token().get();
        let lp_reward_tally = self.lp_reward_tally(&address).get();
        let lp_reward = lp_staked * lp_reward_per_token - lp_reward_tally;

        (core_reward + lp_reward) / BigUint::from(10u64).pow(PRECISION)
    }

    #[storage_mapper("earn:core_stake_token-1")]
    fn core_stake_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("earn:core_stake_total-1")]
    fn core_stake_total(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("earn:core_stake-1")]
    fn core_stake(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[storage_mapper("earn:core_reward_per_token-1")]
    fn core_reward_per_token(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("earn:core_reward_tally-1")]
    fn core_reward_tally(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    // --

    #[storage_mapper("earn:lp_stake_token-1")]
    fn lp_stake_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("earn:lp_stake_total-1")]
    fn lp_stake_total(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("earn:lp_stake-1")]
    fn lp_stake(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[storage_mapper("earn:lp_reward_per_token-1")]
    fn lp_reward_per_token(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("earn:lp_reward_tally-1")]
    fn lp_reward_tally(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;
}
