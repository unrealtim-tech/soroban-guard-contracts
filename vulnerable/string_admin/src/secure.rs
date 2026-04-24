use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

use super::DataKey;

#[contract]
pub struct SecureConfigContract;

#[contractimpl]
impl SecureConfigContract {
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().persistent().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().persistent().set(&DataKey::Admin, &admin);
    }

    /// SECURE: retrieves the stored `Address` and calls `require_auth()`,
    /// which enforces a cryptographic signature check.
    pub fn set_config(env: Env, new_value: u32) {
        let admin: Address = env.storage().persistent().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.storage().persistent().set(&DataKey::Config, &new_value);
    }

    pub fn get_config(env: Env) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::Config)
            .unwrap_or(0)
    }
}
