use std::{
    cell::RefCell,
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{Read, Result as IoResult, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::{block::BlockId, page::Page};

pub struct FileManager {
    db_directory: PathBuf,
    block_size: usize,
    is_new: bool,
    open_files: HashMap<String, Rc<RefCell<File>>>,
}

impl FileManager {
    pub fn new<P: AsRef<Path>>(db_directory: P, block_size: usize) -> IoResult<Self> {
        let db_directory = db_directory.as_ref().to_path_buf();
        let is_new = !db_directory.exists();

        if is_new {
            fs::create_dir_all(&db_directory)?;
        }

        if let Ok(entries) = fs::read_dir(&db_directory) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                    if filename.starts_with("temp") {
                        let _ = fs::remove_file(&path);
                    }
                }
            }
        }

        Ok(Self {
            db_directory,
            block_size,
            is_new,
            open_files: HashMap::new(),
        })
    }

    pub fn read(&mut self, block_id: &BlockId, page: &mut Page) -> IoResult<()> {
        let filename = block_id.filename();
        let block_number = block_id.number() as u64;
        let offset = block_number * self.block_size as u64;

        let file_ref = self.get_file(filename)?;
        let mut file = file_ref.borrow_mut();

        file.seek(SeekFrom::Start(offset))?;
        file.read_exact(page.contents_mut())?;

        Ok(())
    }

    pub fn write(&mut self, block_id: &BlockId, page: &Page) -> IoResult<()> {
        let filename = block_id.filename();
        let block_number = block_id.number() as u64;
        let offset = block_number * self.block_size as u64;

        let file_ref = self.get_file(filename)?;
        let mut file = file_ref.borrow_mut();

        file.seek(SeekFrom::Start(offset))?;
        file.write_all(page.contents())?;

        Ok(())
    }

    pub fn append(&mut self, filename: &str) -> IoResult<BlockId> {
        let new_block_number = self.length(filename)? as i32;
        let block_id = BlockId::new(filename.to_string(), new_block_number);
        let offset = (new_block_number as u64) * (self.block_size as u64);

        let empty_bytes = vec![0u8; self.block_size];

        let file_ref = self.get_file(filename)?;
        let mut file = file_ref.borrow_mut();

        file.seek(SeekFrom::Start(offset))?;
        file.write_all(&empty_bytes)?;

        Ok(block_id)
    }

    pub fn length(&mut self, filename: &str) -> IoResult<usize> {
        let file_ref = self.get_file(filename)?;
        let file = file_ref.borrow();
        let file_len = file.metadata()?.len();
        Ok((file_len / self.block_size as u64) as usize)
    }

    pub fn block_size(&self) -> usize {
        self.block_size
    }

    pub fn is_new(&self) -> bool {
        self.is_new
    }

    fn get_file(&mut self, filename: &str) -> IoResult<Rc<RefCell<File>>> {
        if let Some(file_rc) = self.open_files.get(filename) {
            return Ok(file_rc.clone());
        }

        let file_path = self.db_directory.join(filename);
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;

        let file_rc = Rc::new(RefCell::new(file));
        self.open_files
            .insert(filename.to_string(), file_rc.clone());

        Ok(file_rc)
    }
}
