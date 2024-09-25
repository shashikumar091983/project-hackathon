#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short};

// Define a structure to store metadata for music NFTs
#[contracttype]
#[derive(Clone)]
pub struct MusicNFT {
    pub unique_id: u64,
    pub artist_name: String,
    pub track_title: String,
    pub track_uri: String,  // URI for storing the digital asset (NFT or music file)
    pub royalties: u64,     // Royalty percentage for the artist
    pub price: u64,         // Price for purchasing the track
    pub sold_count: u64,    // Count of how many times the track was sold
}

// Symbol to keep track of total NFTs
const NFT_COUNT: Symbol = symbol_short!("NFT_COUNT");

// Mapping MusicNFT to its unique ID (music_id)
#[contracttype]
pub enum Musicbook {
    MusicNFT(u64),
}

#[contract]
pub struct MusicPlatformContract;

#[contractimpl]
impl MusicPlatformContract {

    // Function to create a new Music NFT
    pub fn create_music_nft(env: Env, artist_name: String, track_title: String, track_uri: String, royalties: u64, price: u64) -> u64 {
        let mut nft_count: u64 = env.storage().instance().get(&NFT_COUNT).unwrap_or(0);
        nft_count += 1;

        // Create a new MusicNFT instance
        let music_nft = MusicNFT {
            unique_id: nft_count,
            artist_name: artist_name.clone(),
            track_title: track_title.clone(),
            track_uri: track_uri.clone(),
            royalties: royalties,
            price: price,
            sold_count: 0,
        };

        // Store the newly created music NFT
        env.storage().instance().set(&Musicbook::MusicNFT(nft_count), &music_nft);

        // Update NFT count in storage
        env.storage().instance().set(&NFT_COUNT, &nft_count);

        log!(&env, "Music NFT created with ID: {}", nft_count);

        nft_count // Return the unique ID of the newly created music NFT
    }

    // Function to purchase a music NFT
    pub fn purchase_music(env: Env, music_id: u64, payment_amount: u64) {
        // Retrieve the music NFT from storage
        let mut music_nft = Self::get_music_nft(env.clone(), music_id);

        // Ensure the payment is equal to or greater than the price
        if payment_amount < music_nft.price {
            log!(&env, "Insufficient payment! Required: {}, Provided: {}", music_nft.price, payment_amount);
            panic!("Insufficient payment!");
        }

        // Distribute the payment (royalties to the artist and platform fee)
        let royalties_paid = payment_amount * music_nft.royalties / 100;
        let platform_fee = payment_amount - royalties_paid;

        log!(&env, "Paid {} to the artist and {} to the platform", royalties_paid, platform_fee);

        // Increment the sold count for the music NFT
        music_nft.sold_count += 1;

        // Update the music NFT in storage
        env.storage().instance().set(&Musicbook::MusicNFT(music_nft.unique_id), &music_nft);

        log!(&env, "Music NFT with ID: {} sold. Total sold: {}", music_id, music_nft.sold_count);
    }

    // Function to view a music NFT by its ID
    pub fn get_music_nft(env: Env, music_id: u64) -> MusicNFT {
        env.storage().instance().get(&Musicbook::MusicNFT(music_id)).unwrap_or(MusicNFT {
            unique_id: 0,
            artist_name: String::from_str(&env, "Not Found"),
            track_title: String::from_str(&env, "Not Found"),
            track_uri: String::from_str(&env, "Not Found"),
            royalties: 0,
            price: 0,
            sold_count: 0,
        })
    }

    // Function to view the total count of NFTs created
    pub fn get_nft_count(env: Env) -> u64 {
        env.storage().instance().get(&NFT_COUNT).unwrap_or(0)
    }
}
