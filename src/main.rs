use std::collections::HashMap;

const NUM_DIRECT_POINTERS: usize = 5;

#[derive(Clone, Debug, PartialEq)]
enum FileType {
    RegularFile,
    Directory,
}

#[derive(Clone, Debug)]
struct Inode {
    id: u64,
    name: String,
    size: u64,
    file_type: FileType,
    direct_pointers: [Option<u64>; NUM_DIRECT_POINTERS],
    entries: Option<Vec<u64>>, // For directories only
    data: Option<Vec<u8>>,     // For storing file content
}

struct JournalEntry {
    operation: String,
    committed: bool,
}

struct Journal {
    entries: Vec<JournalEntry>,
}

impl Journal {
    fn new() -> Self {
        Self { entries: Vec::new() }
    }

    fn log(&mut self, operation: String) {
        self.entries.push(JournalEntry {
            operation,
            committed: true,
        });
    }

    fn undo(&mut self) -> Option<String> {
        if let Some(entry) = self.entries.pop() {
            Some(entry.operation)
        } else {
            None
        }
    }

    fn print_journal(&self) {
        println!("Journal Entries:");
        for (i, entry) in self.entries.iter().enumerate() {
            println!("{}. {} [Committed: {}]", i + 1, entry.operation, entry.committed);
        }
    }
}

struct FileSystem {
    next_id: u64,
    inodes: HashMap<u64, Inode>,
    journal: Journal,
}

impl FileSystem {
    fn new() -> Self {
        Self {
            next_id: 1,
            inodes: HashMap::new(),
            journal: Journal::new(),
        }
    }

    fn create_directory(&mut self, name: &str) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let inode = Inode {
            id,
            name: name.to_string(),
            size: 0,
            file_type: FileType::Directory,
            direct_pointers: [None; NUM_DIRECT_POINTERS],
            entries: Some(Vec::new()),
            data: None,
        };
        self.inodes.insert(id, inode);
        self.journal.log(format!("CREATE DIRECTORY: {}", name));
        id
    }

    fn create_file(&mut self, name: &str) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let inode = Inode {
            id,
            name: name.to_string(),
            size: 0,
            file_type: FileType::RegularFile,
            direct_pointers: [None; NUM_DIRECT_POINTERS],
            entries: None,
            data: None,
        };
        self.inodes.insert(id, inode);
        self.journal.log(format!("CREATE FILE: {}", name));
        id
    }

    fn undo_last_operation(&mut self) {
        if let Some(last_operation) = self.journal.undo() {
            let parts: Vec<&str> = last_operation.split(':').map(|s| s.trim()).collect();
            if parts.len() < 2 {
                println!("Invalid journal entry: {}", last_operation);
                return;
            }

            match parts[0] {
                "WRITE TO FILE" => {
                    if let Ok(file_id) = parts[1].parse::<u64>() {
                        if let Some(file) = self.inodes.get_mut(&file_id) {
                            file.data = None;
                            file.size = 0;
                            println!("Undid write operation on file ID {}", file_id);
                        }
                    }
                }
                _ => println!("Undo not implemented for operation: {}", parts[0]),
            }
        } else {
            println!("Nothing to undo.");
        }
    }


    fn add_file_to_directory(&mut self, file_id: u64, dir_id: u64) {
        if let Some(dir) = self.inodes.get_mut(&dir_id) {
            if let Some(entries) = &mut dir.entries {
                entries.push(file_id);
                self.journal.log(format!("ADD FILE: {} TO DIRECTORY: {}", file_id, dir_id));
            }
        }
    }

    fn write_to_file(&mut self, file_id: u64, data: &[u8]) {
        if let Some(file) = self.inodes.get_mut(&file_id) {
            if file.file_type == FileType::RegularFile {
                file.size = data.len() as u64;
                file.data = Some(data.to_vec());
                self.journal.log(format!("WRITE TO FILE: {}", file_id));
            } else {
                println!("Error: Cannot write to a directory!");
            }
        }
    }

    fn read_file(&self, file_id: u64) -> Vec<u8> {
        if let Some(file) = self.inodes.get(&file_id) {
            if let Some(data) = &file.data {
                return data.clone();
            }
        }
        vec![]
    }

    fn list_directories_and_files(&self) {
        for inode in self.inodes.values() {
            match &inode.file_type {
                FileType::Directory => {
                    println!("Directory {} (ID: {}):", inode.name, inode.id);
                    if let Some(entries) = &inode.entries {
                        for entry_id in entries {
                            if let Some(entry) = self.inodes.get(entry_id) {
                                println!("- File {} (ID: {}, Size: {} bytes)", entry.name, entry.id, entry.size);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_list_directory() {
        let mut fs = FileSystem::new();
        let dir_id = fs.create_directory("Documents");
        assert!(fs.inodes.contains_key(&dir_id));
        assert_eq!(fs.inodes.get(&dir_id).unwrap().name, "Documents");
    }

    #[test]
    fn test_create_and_list_file() {
        let mut fs = FileSystem::new();
        let file_id = fs.create_file("file.txt");
        assert!(fs.inodes.contains_key(&file_id));
        assert_eq!(fs.inodes.get(&file_id).unwrap().name, "file.txt");
    }

    #[test]
    fn test_write_and_read_file() {
        let mut fs = FileSystem::new();
        let file_id = fs.create_file("data.txt");
        fs.write_to_file(file_id, b"Hello, world!");
        let data = fs.read_file(file_id);
        assert_eq!(String::from_utf8_lossy(&data), "Hello, world!");
    }

    #[test]
    fn test_add_file_to_directory() {
        let mut fs = FileSystem::new();
        let dir_id = fs.create_directory("Documents");
        let file_id = fs.create_file("doc.txt");
        fs.add_file_to_directory(file_id, dir_id);
        let dir = fs.inodes.get(&dir_id).unwrap();
        assert!(dir.entries.as_ref().unwrap().contains(&file_id));
    }

    #[test]
    fn test_journal_logging_and_undo() {
        let mut fs = FileSystem::new();
        let dir_id = fs.create_directory("Logs");
        assert_eq!(fs.journal.entries.len(), 1);
        fs.journal.undo();
        assert!(fs.journal.entries.is_empty());
    }

    #[test]
    fn test_undo_last_operation() {
        let mut fs = FileSystem::new();
        let file_id = fs.create_file("temp.txt");
        fs.write_to_file(file_id, b"Temporary data");
    
        // Undo the last operation (write)
        fs.undo_last_operation();
    
        // Ensure the file is still present but has no data
        let file = fs.inodes.get(&file_id).unwrap();
        assert!(file.data.is_none());
        assert_eq!(file.size, 0);
    }
}

fn main() {
    let mut fs = FileSystem::new();

    let dir1 = fs.create_directory("Documents");
    let dir2 = fs.create_directory("Pictures");
    let file1 = fs.create_file("doc1.txt");
    let file2 = fs.create_file("doc2.txt");
    let file3 = fs.create_file("pic1.jpg");

    fs.add_file_to_directory(file1, dir1);
    fs.add_file_to_directory(file2, dir1);
    fs.add_file_to_directory(file3, dir2);

    fs.write_to_file(file1, b"Hello, world!");

    println!("\n=== Directory Listing ===");
    fs.list_directories_and_files();

    let data = fs.read_file(file1);
    println!("\n=== Read File ===");
    println!("File Data: {}", String::from_utf8_lossy(&data));

    println!("\n=== Journal ===");
    fs.journal.print_journal();

    println!("\n=== Undo Operation ===");
    if let Some(undone_operation) = fs.journal.undo() {
        println!("Undid operation: {}", undone_operation);
    } else {
        println!("Nothing to undo.");
    }

    println!("\n=== Final Journal ===");
    fs.journal.print_journal();
}
