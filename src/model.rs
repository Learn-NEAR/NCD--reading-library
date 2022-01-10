use std::collections::HashMap;

use near_sdk::{
    env,
    borsh::{
        self,
        BorshDeserialize,
        BorshSerialize,
    },
    serde::{
        Deserialize,
        Serialize,
    },
};

use crate::utils::AccountId;

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum RateEnum{
    Bad,
    Regular,
    Awesome,
    None,
}

impl Default for RateEnum{
    fn default() -> Self {
        RateEnum::None
    }
}

#[derive(Clone, Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct BookInformation{
    id: u64,
    owner: String,
    isbn: String,
    name: String,
    description: String,
    numpage: u64,
    author: String,
    datepublished: String,
    editions: u64,
    rates: Vec<Rate>,
    comments: HashMap<AccountId, String>,
    timestamp: u64,
}

impl BookInformation{
    pub fn new(
        id: u64,
        isbn: String,
        name: String,
        description: String,
        numpage: u64,
        author: String,
        datepublished: String,
        editions: u64,
        timestamp: u64,
    )-> Self{
        // Assemblyscript is using context.sender as owner
        let owner: String = env::signer_account_id();
        BookInformation{
            id,
            owner,
            isbn,
            name,
            description,
            numpage,
            author,
            datepublished,
            editions,
            rates: Vec::new(),
            comments: HashMap::new(),
            timestamp,
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Rate{
    owner: String,
    rate: RateEnum,
}

impl Rate{
    pub fn new(owner: String, rate: RateEnum) -> Self {
        Rate{
            owner,
            rate,
        }
    }
}
