#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Address};

#[contracttype]
pub enum DataKey {
    Payment(Address),
}

#[contract]
pub struct PayLinyaContract;

#[contractimpl]
impl PayLinyaContract {

    // Student makes a payment
    pub fn pay(env: Env, student: Address, amount: i128) {
        student.require_auth();

        env.storage()
            .instance()
            .set(&DataKey::Payment(student.clone()), &amount);
    }

    // Get payment amount
    pub fn get_payment(env: Env, student: Address) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Payment(student))
            .unwrap_or(0)
    }

    // Check if paid
    pub fn verify_payment(env: Env, student: Address) -> bool {
        Self::get_payment(env, student) > 0
    }
}