
## Version 1: Basic Key-Value Store

### Architecture

#### Components
1. **CLI (Command-Line Interface)**
   - Handles user input and provides interaction with the database.
   - Commands: `insert`, `get`, `update`, `delete`, `exit`.

2. **Key-Value Store**
   - Core logic for storing and managing key-value pairs.
   - Uses in-memory HashMap for data manipulation.
   - Provides methods for CRUD operations.

3. **Storage Layer**
   - Handles persistent storage of key-value pairs.
   - Uses file-based storage.
   - Serializes data to JSON for saving.
   - Deserializes JSON for loading.

#### Data Flow

1. **User Interaction**
   - User inputs command through the CLI.

2. **Command Parsing**
   - CLI module parses the command.

3. **CRUD Operations**
   - CLI module calls the appropriate method in the Key-Value Store module.

4. **Data Manipulation**
   - Key-Value Store manipulates the in-memory HashMap.
   - For `insert`, `update` and `delete`, it also calls the Storage Layer to persist changes.

5. **Storage Persistence**
   - Storage Layer serializes the HashMap to JSON.
   - Saves JSON to a file for persistence.

6. **Data Retrieval**
   - On startup or load command, the Storage Layer deserializes JSON from the file.
   - Populates the HashMap in the Key-Value Store.
### Usage:
```shell
$> cargo run
(safina_db) ➜ insert "name" "Abdellah A."
Inserted entry {'name' : 'Abdellah A.'}

(safina_db) ➜ get name
Entry: name = Abdellah A.

(safina_db) ➜ exit
exit...
```
### Diagram

<img src="../ressources/version1-diagram.png" alt="v1-diagram" width="900">


### Roadmap


1. **Implement Basic Storage Functions**:
    - [x] Implement the `insert`.
    - [x] Implement the `get`.
    - [ ] Implement the `update`.
    - [ ] Implement the `delete`.


2. **Set Up Command-Line Interface**:
    - [x] Add dependencies in `Cargo.toml` for the `clap` crate.
    - [x] Create a basic CLI for inserting and retrieving data.

4. **Persistence**:
    - [ ] Implement simple file-based persistence to save and load key-value pairs.
        - [ ] Define a method to save the key-value pairs to a file.
        - [ ] Define a method to load key-value pairs from a file at startup.
    - [ ] Ensure data is loaded from the file on startup and saved on exit.
        - [ ] Modify the `main` function to load data at startup.
        - [ ] Modify the `main` function to save data on exit.
5.  **Testing**
    - [ ] Write unit tests for CRUD operations and storage functions.
      - [ ] Test insertion of key-value pairs.
      - [ ] Test retrieval of values by key.
      - [ ] Test updating values for existing keys.
      - [ ] Test deletion of key-value pairs.
      - [ ] Test serialization and deserialization of the HashMap.
  