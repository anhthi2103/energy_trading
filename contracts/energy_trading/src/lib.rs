#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Map};

#[contract]
pub struct EnergyTrading;

#[contractimpl]
impl EnergyTrading {
    /// List energy for sale by an energy producer.
    /// Returns the listing ID.
    pub fn list_energy(env: Env, producer: Address, amount: u64, price_per_unit: u64) -> u64 {
        producer.require_auth();

        let mut listings: Map<u64, (Address, u64, u64, u64)> = env
            .storage()
            .instance()
            .get(&"listings")
            .unwrap_or(Map::new(&env));

        let listing_id = listings.keys().len() as u64;
        listings.set(listing_id, (producer.clone(), amount, price_per_unit, amount));
        env.storage().instance().set(&"listings", &listings);

        // Update total listed
        let mut total_listed: u64 = env
            .storage()
            .instance()
            .get(&"total_listed")
            .unwrap_or(0);
        total_listed += amount;
        env.storage().instance().set(&"total_listed", &total_listed);

        listing_id
    }

    /// Buy energy from a listing.
    /// Returns the total cost.
    pub fn buy_energy(env: Env, consumer: Address, listing_id: u64, amount: u64) -> u64 {
        consumer.require_auth();

        let mut listings: Map<u64, (Address, u64, u64, u64)> = env
            .storage()
            .instance()
            .get(&"listings")
            .unwrap_or(Map::new(&env));

        let (producer, original_amount, price_per_unit, remaining) = listings
            .get(listing_id)
            .unwrap_or_else(|| panic!("Listing not found"));

        if amount > remaining {
            panic!("Insufficient energy available");
        }

        let new_remaining = remaining - amount;
        listings.set(listing_id, (producer, original_amount, price_per_unit, new_remaining));
        env.storage().instance().set(&"listings", &listings);

        // Update total listed
        let mut total_listed: u64 = env
            .storage()
            .instance()
            .get(&"total_listed")
            .unwrap_or(0);
        if total_listed >= amount {
            total_listed -= amount;
        }
        env.storage().instance().set(&"total_listed", &total_listed);

        amount * price_per_unit
    }

    /// Get listing details.
    /// Returns (producer, amount, price_per_unit, remaining).
    pub fn get_listing(env: Env, listing_id: u64) -> (Address, u64, u64, u64) {
        let listings: Map<u64, (Address, u64, u64, u64)> = env
            .storage()
            .instance()
            .get(&"listings")
            .unwrap_or(Map::new(&env));

        listings.get(listing_id).unwrap_or_else(|| panic!("Listing not found"))
    }

    /// Get total energy units currently listed.
    pub fn get_total_listed(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&"total_listed")
            .unwrap_or(0)
    }
}
