#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, String,
};

#[contracttype]
#[derive(Clone)]
pub struct PaymentRequest {
    pub id: u64,
    pub merchant: Address,
    pub amount: i128,
    pub currency: String,
    pub paid: bool,
    pub payer: Option<Address>,
}

#[contracttype]
pub enum DataKey {
    Payment(u64),
    Count,
}

#[contract]
pub struct StellarQrPay;

#[contractimpl]
impl StellarQrPay {
    pub fn project_name(env: Env) -> String {
        String::from_str(&env, "Stellar QR Pay")
    }

    pub fn create_payment(
        env: Env,
        merchant: Address,
        amount: i128,
        currency: String,
    ) -> u64 {
        merchant.require_auth();

        let mut count: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::Count)
            .unwrap_or(0);

        count += 1;

        let payment = PaymentRequest {
            id: count,
            merchant: merchant.clone(),
            amount,
            currency: currency.clone(),
            paid: false,
            payer: None,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Payment(count), &payment);

        env.storage()
            .persistent()
            .set(&DataKey::Count, &count);

        env.events().publish(
            (symbol_short!("created"), count),
            (merchant, amount, currency),
        );

        count
    }

    pub fn pay(env: Env, payment_id: u64, payer: Address) -> bool {
        payer.require_auth();

        let mut payment: PaymentRequest = env
            .storage()
            .persistent()
            .get(&DataKey::Payment(payment_id))
            .expect("Payment not found");

        if payment.paid {
            panic!("Payment already paid");
        }

        payment.paid = true;
        payment.payer = Some(payer.clone());

        env.storage()
            .persistent()
            .set(&DataKey::Payment(payment_id), &payment);

        env.events().publish(
            (symbol_short!("paid"), payment_id),
            (payer, payment.amount, payment.currency),
        );

        true
    }

    pub fn get_payment(env: Env, payment_id: u64) -> PaymentRequest {
        env.storage()
            .persistent()
            .get(&DataKey::Payment(payment_id))
            .expect("Payment not found")
    }

    pub fn is_paid(env: Env, payment_id: u64) -> bool {
        let payment: PaymentRequest = env
            .storage()
            .persistent()
            .get(&DataKey::Payment(payment_id))
            .expect("Payment not found");

        payment.paid
    }

    pub fn total_payments(env: Env) -> u64 {
        env.storage()
            .persistent()
            .get(&DataKey::Count)
            .unwrap_or(0)
    }
}