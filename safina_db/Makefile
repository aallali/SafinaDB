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
	@cargo run

# Run tests
test:
	@echo "Running tests..."
	@cargo test


# Run benchmarks
bench:
	@echo "Running benchmarks..."
	@cargo bench
	@echo "Moving benchmarks output to 'benchmarks' folder..."
	@rm -rf ../benchmarks
	@mv target/criterion ../benchmarks/

# Generate documentation and move it to 'documentation' folder
doc: 
	@echo "Generating documentation..."
	@cargo doc --no-deps
	@echo "Moving documentation to 'documentation' folder..."
	@rm -rf ../documentation
	@mv target/doc ../documentation

# Clean the project
clean:
	@echo "Cleaning the project..."
	@cargo clean
	@rm -rf documentation
