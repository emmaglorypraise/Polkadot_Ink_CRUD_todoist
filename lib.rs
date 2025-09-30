#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod crud_todo {
    use ink::storage::Mapping;
    use ink::prelude::string::String;

    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        TodoNotFound,
        TodoAlreadyExists,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]

    pub struct Todo {
        id: u32,
        title: String,
        status: bool,
    }

    #[ink(storage)]
    pub struct TodoContract {
        todos: Mapping<u32, Todo>,
        next_id: u32,
    }

    impl Default for TodoContract {
            fn default() -> Self {
            Self::new()
            }
    }
    
    impl TodoContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                todos: Mapping::new(),
                next_id: 1,
            }
        }

        #[ink(message)]
        pub fn create(&mut self, title: String) -> Result<u32, Error> {
            let id = self.next_id;
            let todo = Todo {
                id,
                title,
                status: false,
            };
            
            self.todos.insert(id, &todo);
            self.next_id = self.next_id.saturating_add(1);
            
            Ok(id)
        }
    
        #[ink(message)]
        pub fn read(&self, id: u32) -> Result<Todo, Error> {
            self.todos.get(id).ok_or(Error::TodoNotFound)
        }

        #[ink(message)]
        pub fn update(&mut self, id: u32, title: Option<String>, status: Option<bool>) -> Result<(), Error> {
            let mut todo = self.todos.get(id).ok_or(Error::TodoNotFound)?;
            
            if let Some(new_title) = title {
                todo.title = new_title;
            }
            
            if let Some(new_status) = status {
                todo.status = new_status;
            }
            
            self.todos.insert(id, &todo);
            Ok(())
        }

        #[ink(message)]
        pub fn delete(&mut self, id: u32) -> Result<(), Error> {
            if self.todos.get(id).is_none() {
                return Err(Error::TodoNotFound);
            }
            
            self.todos.remove(id);
            Ok(())
        }

        /// Get the next available ID
        #[ink(message)]
        pub fn get_next_id(&self) -> u32 {
            self.next_id
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn create_todo() {
            let mut contract = TodoContract::new();
            let id = contract.create(String::from("Write Rust")).unwrap();
            assert_eq!(id, 1);
            
            let todo = contract.read(id).unwrap();
            assert_eq!(todo.title, "Write Rust");
            assert_eq!(todo.status, false);
        }

        #[ink::test]
        fn read_todo() {
            let mut contract = TodoContract::new();
            let id = contract.create(String::from("Test todo")).unwrap();
            let todo = contract.read(id).unwrap();
            assert_eq!(todo.id, 1);
            assert_eq!(todo.title, "Test todo");
        }

        #[ink::test]
        fn update_todo() {
            let mut contract = TodoContract::new();
            let id = contract.create(String::from("Old title")).unwrap();
            
            contract.update(id, Some(String::from("New title")), Some(true)).unwrap();
            let todo = contract.read(id).unwrap();
            assert_eq!(todo.title, "New title");
            assert_eq!(todo.status, true);
        }

        #[ink::test]
        fn delete_todo() {
            let mut contract = TodoContract::new();
            let id = contract.create(String::from("To be deleted")).unwrap();
            
            contract.delete(id).unwrap();
            assert_eq!(contract.read(id), Err(Error::TodoNotFound));
        }

        #[ink::test]
        fn error_handling_todo() {
            let mut contract = TodoContract::new();
            assert_eq!(contract.read(999), Err(Error::TodoNotFound));
            assert_eq!(contract.delete(999), Err(Error::TodoNotFound));
        }
    }
}