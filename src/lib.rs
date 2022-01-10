mod utils;
mod model;

// use std::collections::HashMap;

use near_sdk::{
    env,
    near_bindgen,
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

use crate::model::BookInformation;


near_sdk::setup_alloc!();

const MAX_DESCRIPTION_LENGTH: usize = 255;
const MAX_BOOKPAGE_LENGTH: u64 = 1200;

#[near_bindgen]
#[derive(Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Contract{
    books: Vec<BookInformation>,
    // This is declared in assemblyscript but not used.
    // rateL: HashMap<AccountId, u32>,
}

#[near_bindgen]
impl Contract{
    #[allow(non_snake_case)]
    pub fn AddBook(
        &mut self,
        isbn: String,
        name: String,
        description: String,
        numpage: u64,
        author: String,
        datepublished: String,
        editions: u64,
    ){
        assert!(isbn.len() > 0, "The ISBN is required.");
        assert!(name.len() > 0, "The name is required.");
        assert!(description.len() > 0, "The description is required.");
        assert!(description.len() < MAX_DESCRIPTION_LENGTH, "Exceeded character's number.");
        assert!(numpage > 0, "The numpage is required.");
        assert!(numpage < MAX_BOOKPAGE_LENGTH, "Exceeded page quantity.");
        assert!(author.len() > 0, "The author is required.");
        assert!(datepublished.len() > 0, "The datepublished is required.");
        assert!(editions > 0, "The editions is required.");

        let id: u64 = self.books.len() as u64;
        let timestamp: u64 = env::block_timestamp();

        let book: BookInformation = BookInformation::new(
            id,                                 // id: u64,
            isbn,                               // isbn: String,
            name,                               // name: String,
            description,                        // description: String,
            numpage,                            // numpage: u64,
            author,                             // author: String,
            datepublished,                      // datepublished: String,
            editions,                           // editions: u64,
            timestamp,                          // timestamp: u64,
        );

        self.books.push(book);
    }

    #[allow(non_snake_case)]
    pub fn getBooks(&self) -> Vec<BookInformation> {
        let books_len: usize = self.books.len();
        let mut result: Vec<BookInformation> = Vec::with_capacity(books_len);

        for i in 0..books_len{
            // get(i) only returns a reference, we have to clone it before returning.
            let book: BookInformation = self.books
                .get(i)
                .unwrap()
                .clone();

            result.push(book)
        };

        result
    }

    #[allow(non_snake_case)]
    pub fn getBook(&self, id: i32) -> BookInformation {
        let books_len: usize = self.books.len();
        assert!(books_len > 0, "We haven't any Books");
        assert!(id >= 0, "ID must be positive");
        assert!(id as usize <= books_len, "We Haven't that Book");

        // get(id) only returns a reference, we have to clone it before returning.
        let book: BookInformation = self.books
            .get(id as usize)
            .unwrap()
            .clone();
        
        book
    }

    #[allow(non_snake_case)]
    pub fn getNBooks(&self) -> usize {
        return self.books.len()
    }

}

#[cfg(test)]
mod tests{
    use super ::*;

    use near_sdk::{testing_env, VMContext, MockedBlockchain};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {

        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }


    // These are constant values so it's easy to see what is different in each test.
    // Using functions to return these values because the compiler doesn't like the idea of constant Strings.
    #[allow(non_snake_case)]
    fn ISBN() -> String { String::from("123-4564-54-4") }
    #[allow(non_snake_case)]
    fn NAME() -> String { String::from("Test") }
    #[allow(non_snake_case)]
    fn DESCRIPTION() -> String { String::from("Description test") }
    #[allow(non_snake_case)]
    fn NUMPAGE() -> u64 { 123 }
    #[allow(non_snake_case)]
    fn AUTHOR() -> String { String::from("juan perez") }
    #[allow(non_snake_case)]
    fn DATEPUBLISHED() -> String { String::from("marzo 2099") }
    #[allow(non_snake_case)]
    fn EDITIONS() -> u64 { 1 } 
    

    #[test]
    pub fn add_book_correct(){
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract: Contract = Contract::default();
        println!("Add a new book Tests.");
        println!("Should run fine with this config");
        
        contract.AddBook(
            ISBN(),                             // isbn: String, 
            NAME(),                             // name: String, 
            DESCRIPTION(),                      // description: String, 
            NUMPAGE(),                          // numpage: u64, 
            AUTHOR(),                           // author: String, 
            DATEPUBLISHED(),                    // datepublished: String, 
            EDITIONS(),                         // editions: u64,
        );
    }

    #[test]
    #[should_panic]
    pub fn add_book_empty_isbn(){
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract: Contract = Contract::default();
        println!("Add a new book Tests.");
        println!("Should fail if we give it an empty ISBN");

        contract.AddBook(
            String::from(""),                   // isbn: String, 
            NAME(),                             // name: String, 
            DESCRIPTION(),                      // description: String, 
            NUMPAGE(),                          // numpage: u64, 
            AUTHOR(),                           // author: String, 
            DATEPUBLISHED(),                    // datepublished: String, 
            EDITIONS(),                         // editions: u64,
        );
    }

    #[test]
    #[should_panic]
    pub fn add_book_empty_name() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract: Contract = Contract::default();
        println!("Add a new book Tests.");
        println!("Should fail if we give it an empty name");

        contract.AddBook(
            ISBN(),                             // isbn: String, 
            String::from(""),                   // name: String, 
            DESCRIPTION(),                      // description: String, 
            NUMPAGE(),                          // numpage: u64, 
            AUTHOR(),                           // author: String, 
            DATEPUBLISHED(),                    // datepublished: String, 
            EDITIONS(),                         // editions: u64,
        );
    }

    #[test]
    #[should_panic]
    pub fn add_book_empty_description() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract: Contract = Contract::default();
        println!("Add a new book Tests.");
        println!("Should fail if we give it an empty description");

        contract.AddBook(
            ISBN(),                             // isbn: String, 
            NAME(),                             // name: String, 
            String::from(""),                   // description: String, 
            NUMPAGE(),                          // numpage: u64, 
            AUTHOR(),                           // author: String, 
            DATEPUBLISHED(),                    // datepublished: String, 
            EDITIONS(),                         // editions: u64,
        );
    }

    #[test]
    #[should_panic]
    pub fn add_book_empty_numpage() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract: Contract = Contract::default();
        println!("Add a new book Tests.");
        println!("Should fail if we give it an empty numpage");

        contract.AddBook(
            ISBN(),                             // isbn: String, 
            NAME(),                             // name: String, 
            DESCRIPTION(),                      // description: String, 
            0,                                  // numpage: u64, 
            AUTHOR(),                           // author: String, 
            DATEPUBLISHED(),                    // datepublished: String, 
            EDITIONS(),                         // editions: u64,
        );
    }

    #[test]
    pub fn get_book() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract: Contract = Contract::default();
        println!("Get a specific book by id");
        println!("Should run fine with this config");

        contract.AddBook(
            ISBN(),                             // isbn: String, 
            NAME(),                             // name: String, 
            DESCRIPTION(),                      // description: String, 
            NUMPAGE(),                          // numpage: u64, 
            AUTHOR(),                           // author: String, 
            DATEPUBLISHED(),                    // datepublished: String, 
            EDITIONS(),                         // editions: u64,
        );


        contract.AddBook(
            ISBN(),                             // isbn: String, 
            NAME(),                             // name: String, 
            DESCRIPTION(),                      // description: String, 
            NUMPAGE(),                          // numpage: u64, 
            AUTHOR(),                           // author: String, 
            DATEPUBLISHED(),                    // datepublished: String, 
            EDITIONS(),                         // editions: u64,
        );

        contract.getBook(1);
    }

    #[test]
    #[should_panic]
    pub fn get_book_negative_id() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let contract: Contract = Contract::default();
        println!("Get a specific book by id");
        println!("Should fail if we give it a negative id.");

        contract.getBook(-1);
    }

    #[test]
    #[should_panic]
    pub fn get_book_out_of_bounds() {
        let context = get_context(vec![], false);
        testing_env!(context);
        
        let contract: Contract = Contract::default();
        println!("Get a specific book by id");
        println!("Should fail if we give it an index out of bounds.");

        contract.getBook(1600);
    }
}
