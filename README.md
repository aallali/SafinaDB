<p align="center">
   <img src="./ressources/Safina DB.png" alt="safinaDbLogo" width="400"/>
</p>

# SafinaDB 🚢

SafinaDB (safina;سفينة; ship; saa-fee-na) is a custom-built database engine designed to be efficient, stable annd fast **(for learning purposes)**. Inspired by the Arabic word for "ship".
This project is developed incrementally, starting from a basic key-value store to an advanced distributed database system.


## 📋 Table of contents
- [Project Roadmap](#️-project-roadmap)
- [Getting Started](#-getting-started)
- [Contributing](#Contributing)


---

## 🗺️ Project Roadmap

### [Version 1: Basic Key-Value Store](./roadmaps/version1.md) ⏳
- Data Storage: File-based storage for key-value pairs.
- Basic CRUD Operations: Create, Read, Update, Delete functionality.
- Simple CLI: Command-line interface for interacting with the database.

### Version 2: In-Memory Cache
- In-Memory Data Structure: Add an in-memory hash map for faster access.
- Write-Through Cache: Implement caching with write-through to disk.
- Basic Indexing: Simple in-memory indexing for fast lookups.

### Version 3: Persistence and Recovery
- Log-based Storage: Implement write-ahead logging for durability.
- Recovery Mechanism: Add crash recovery to replay logs.
- Data Serialization: Efficient serialization and deserialization of data.

### Version 4: Advanced Indexing
- B-Tree Indexes: Implement B-tree indexing for efficient range queries.
- Secondary Indexes: Add support for secondary indexes.
- Query Optimization: Basic query optimization techniques.

### Version 5: Concurrency Control
- Locking Mechanism: Implement basic locking for concurrent access.
- Transactions: Support for transactions with ACID properties.
- Isolation Levels: Implement different transaction isolation levels.

### Version 6: Replication
- Master-Slave Replication: Basic replication for high availability.
- Data Synchronization: Mechanism to sync data between nodes.
- Fault Tolerance: Basic fault tolerance and failover.

### Version 7: Advanced Features
- Query Language: Implement a simple query language (e.g., SQL-like syntax).
- Stored Procedures: Support for stored procedures and functions.
- User Authentication: Basic user authentication and access control.

## [future goals] Will keep them here for motivation.
### *Version 8: Performance Optimization*
- Caching Strategies: Implement advanced caching strategies.
- Compression: Data compression for efficient storage.
- Performance Tuning: Optimize data structures and algorithms for performance.

### *Version 9: Distributed Database*
- Sharding: Implement data sharding across multiple nodes.
- Consistency Protocols: Implement consistency protocols like Quorum or Paxos.
- Distributed Transactions: Support for distributed transactions and two-phase commit.

### *Version 10: Final Refinements*
- Comprehensive Testing: Extensive testing for stability and performance.
- Documentation: Detailed documentation and user guides.
- Release: Finalize the release and publish as an open-source project.

## 🏁 Getting Started

To get started with SafinaDB, follow these steps:

1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/SafinaDB.git
   cd SafinaDB/app
   ```

2. **Build the project**:
   ```bash
   make build
   ```

3. **Run the database server**:
   ```bash
   ./safinadb start
   ```

4. **Use the CLI**:
   ```bash
   ./safinadb-cli
   ```

## Contributing

Contributions are welcome! Please fork this repository and submit pull requests to add new features or fix bugs. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
