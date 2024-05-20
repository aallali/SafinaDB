
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

#### Modules

1. **Main Module (`main.rs`)**
   - Initializes the CLI.
   - Manages the interaction between the CLI and the Key-Value Store.

2. **CLI Module (`cli.rs`)**
   - Defines functions for parsing and executing commands.
   - Interacts with the Key-Value Store to perform operations.

3. **Key-Value Store Module (`kv_store.rs`)**
   - Contains the HashMap for in-memory storage.
   - Implements CRUD operations.
   - Interfaces with the Storage Layer for persistence.

4. **Storage Module (`storage.rs`)**
   - Implements functions for saving data to a file.
   - Implements functions for loading data from a file.
   - Uses `serde` for serialization/deserialization.

#### Data Flow

1. **User Interaction**
   - User inputs command through the CLI.

2. **Command Parsing**
   - CLI module parses the command.

3. **CRUD Operations**
   - CLI module calls the appropriate method in the Key-Value Store module.

4. **Data Manipulation**
   - Key-Value Store manipulates the in-memory HashMap.
   - For `insert` and `update`, it also calls the Storage Layer to persist changes.

5. **Storage Persistence**
   - Storage Layer serializes the HashMap to JSON.
   - Saves JSON to a file for persistence.

6. **Data Retrieval**
   - On startup or load command, the Storage Layer deserializes JSON from the file.
   - Populates the HashMap in the Key-Value Store.

### Diagram

```
User
  |
  v
CLI
  |
  v
Key-Value Store
      |
  -----------
  |         |
  v         v
In-Memory   Storage
HashMap     - Serialize to JSON
            - File I/O
```
<img src="../ressources/version1-diagram.png" alt="v1-diagram" width="900">


### Roadmap

#### To-Do List for Version 1

- [ ] **Project Setup**
  - [ ] Initialize a new Rust project with Cargo.
    - Run `cargo new safina_db` to create a new project.
  - [ ] Organize project structure (src, tests).
    - Create `src/cli.rs`, `src/kv_store.rs`, and `src/storage.rs`.

- [ ] **Basic Data Storage**
  - [ ] Implement file-based storage for key-value pairs.
    - Create a function to write key-value pairs to a file.
    - Create a function to read key-value pairs from a file.
  - [ ] Use `serde` for serialization and deserialization.
    - Add `serde` and `serde_json` dependencies to `Cargo.toml`.
    - Implement serialization of the HashMap to JSON.
    - Implement deserialization of JSON to the HashMap.

- [ ] **CRUD Operations**
  - [ ] Implement Create operation.
    - Add a method to insert a key-value pair into the HashMap.
    - Save the updated HashMap to the file.
  - [ ] Implement Read operation.
    - Add a method to retrieve a value by key from the HashMap.
  - [ ] Implement Update operation.
    - Add a method to update the value for an existing key in the HashMap.
    - Save the updated HashMap to the file.
  - [ ] Implement Delete operation.
    - Add a method to remove a key-value pair from the HashMap.
    - Save the updated HashMap to the file.

- [ ] **Simple CLI**
  - [ ] Create a command-line interface for interacting with the database.
    - Implement command parsing for `insert`, `get`, `update`, `delete`, `exit`.
    - Connect CLI commands to the respective CRUD operations in the Key-Value Store.

- [ ] **Testing**
  - [ ] Write unit tests for CRUD operations and storage functions.
    - Test insertion of key-value pairs.
    - Test retrieval of values by key.
    - Test updating values for existing keys.
    - Test deletion of key-value pairs.
    - Test serialization and deserialization of the HashMap.

- [ ] **Documentation**
  - [ ] Document setup, usage, and basic architecture in the README.
    - Provide instructions for setting up the project.
    - Provide usage examples for each CLI command.
    - Describe the architecture and data flow.
