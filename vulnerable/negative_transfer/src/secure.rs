use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env};
use super::{get_balance, set_balance};

#[contract]
pub struct SecureTokenContract;

#[contractimpl]
impl SecureTokenContract {
    pub fn mint(env: Env, to: Address, amount: i128) {
        let current = get_balance(&env, &to);
        set_balance(&env, &to, current + amount);
    }

    /// SECURE: rejects `amount <= 0` before touching balances.
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        assert!(amount > 0, "amount must be positive");
        set_balance(&env, &from, get_balance(&env, &from) - amount);
        set_balance(&env, &to, get_balance(&env, &to) + amount);
        env.events()
            .publish((symbol_short!("transfer"),), (from, to, amount));
    }

    pub fn balance(env: Env, account: Address) -> i128 {
        get_balance(&env, &account)
    }
}
