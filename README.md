# Simple File System

This project implements a basic file system with journaling in Rust. It supports creating files and directories, writing and reading files, listing directory contents, and undoing operations using a journal system.

## Setting Up the Project

1. **Clone the Repository**:
   ```bash
   git clone [<repository-url>](https://github.com/JeremiahHerring/simple_file_system.git)
   cd simple_file_system
   ```

2. **Ensure Cargo is Available**:
   Make sure you have Rust and Cargo installed. You can check by running:
   ```bash
   rustc --version
   cargo --version
   ```

3. **Build the Project**:
   Build the project to verify there are no compilation errors:
   ```bash
   cargo build
   ```

4. **Run the Application**:
   Execute the main program:
   ```bash
   cargo run
   ```

## Running Tests

1. **Run All Tests**:
   To ensure all functionalities are working correctly, run:
   ```bash
   cargo test
   ```

2. **Expected Output**:
   If all tests pass, the output will indicate successful execution:
   ```plaintext
   running 6 tests
   test tests::test_add_file_to_directory ... ok
   test tests::test_create_and_list_directory ... ok
   test tests::test_create_and_list_file ... ok
   test tests::test_journal_logging_and_undo ... ok
   test tests::test_write_and_read_file ... ok
   test tests::test_undo_last_operation ... ok

   test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
   ```

3. **Debugging Failed Tests**:
   If any test fails, re-run with backtrace to identify the issue:
   ```bash
   RUST_BACKTRACE=1 cargo test
   ```

## Project Features

### Core Functionalities
- **File and Directory Management**:
  - Create directories and files.
  - Add files to directories.
- **Read/Write Operations**:
  - Write data to files.
  - Read data from files.
- **Directory Listings**:
  - List all files and directories in the file system.
- **Journaling**:
  - Logs all operations.
  - Undo the last operation.

### Example Workflow
1. **Create Directories and Files**:
   ```rust
   let dir_id = fs.create_directory("Documents");
   let file_id = fs.create_file("notes.txt");
   ```

2. **Write and Read Files**:
   ```rust
   fs.write_to_file(file_id, b"Hello, world!");
   let content = fs.read_file(file_id);
   println!("File Content: {}", String::from_utf8_lossy(&content));
   ```

3. **Undo an Operation**:
   ```rust
   fs.journal.undo();
   ```
