# Makefile for Rust project

.PHONY: all build run test bench doc movedoc clean

# Default target
all: build

# Build the project
build:
	@echo "Building the project..."
	@cargo build

# Run the project
run:
	@echo "Running the project..."
	@cargo run -q

# Run tests
test:
	@echo "Running tests..."
	make clean-test-db
	@cargo test
	make clean-test-db

clean-test-db:
	find . -name 'db-test-*' -exec rm -f {} +
# Run benchmarks
bench:
	@echo "Running benchmarks..."
	@cargo bench
	@echo "Moving benchmarks output to 'benchmarks' folder..."
	@rm -rf ../benchmarks
	@mv target/criterion ../benchmarks/
	make clean-test-db

# Generate documentation and move it to 'documentation' folder
doc: 
	@echo "Generating documentation..."
	@cargo doc --no-deps
	@echo "Moving documentation to 'documentation' folder..."
	@rm -rf ../documentation
	@mv target/doc ../documentation
	@open ../documentation/safina_db/index.html

# Clean the project
clean:
	@echo "Cleaning the project..."
	@cargo clean
	@rm -rf documentation
	make clean-test-db
