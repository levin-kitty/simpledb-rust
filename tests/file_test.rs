use std::{fs::remove_dir_all, panic, path::Path};

use simpledb_rust::{block::BlockId, filemanager::FileManager, page::Page};

#[test]
fn file_test() {
    let result = panic::catch_unwind(|| {
        let mut file_manager = FileManager::new("filetest", 400).unwrap();
        let block_id = BlockId::new("testfile", 2);

        let mut page1 = Page::new(file_manager.block_size());
        let pos1 = 88;

        let input_string = "abcdefghijklm";
        let input_int = 345;

        page1.set_string(pos1, input_string);
        let size = Page::max_length(input_string.len());
        let pos2 = pos1 + size;

        page1.set_int(pos2, input_int);
        file_manager.write(&block_id, &page1).unwrap();

        let mut page2 = Page::new(file_manager.block_size());
        file_manager.read(&block_id, &mut page2).unwrap();

        let read_string = page2.get_string(pos1);
        let read_int = page2.get_int(pos2);

        assert_eq!(read_string, input_string);
        assert_eq!(read_int, input_int);
    });

    let test_dir = Path::new("filetest");
    if test_dir.exists() {
        remove_dir_all(test_dir).unwrap();
    }

    result.unwrap_or_else(|_| panic!("Test failed"));
}
