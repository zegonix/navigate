use std::fs;
use std::fs::File;
use std::io::{Error, Result};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use sysinfo::{Pid, System};

#[derive(Debug, Clone)]
pub struct Stack {
    pid: u32,
    path: PathBuf,
    stack: Vec<PathBuf>,
}

impl Stack {
    pub fn new(process_id: u32) -> Result<Self> {
        let mut stack: Stack = Stack {
            pid: process_id,
            path: PathBuf::new(),
            stack: Vec::<PathBuf>::new(),
        };

        // remove first entry if it is empty, because after
        // creation of the stack there seems to be an empty
        // cell in the vector
        stack.build_stack()?;
        if !stack.stack[0].is_dir() {
            stack.stack.remove(0);
        }

        Ok(stack)
    }

    // return stack
    pub fn get_stack(&mut self) -> Result<&Vec<PathBuf>> {
        Ok(&self.stack)
    }

    /// clear stack by deleting the associated stack file
    pub fn clear_stack(&mut self) -> Result<()> {
        fs::remove_file(self.path.clone())
    }

    /// push entry to stack
    /// returns updated stack
    pub fn push_entry(&mut self, path: &Path) -> Result<&Vec<PathBuf>> {
        let abs_path = path.canonicalize()?;
        self.stack.push(abs_path);
        self.write_stack_file()?;
        Ok(&self.stack)
    }

    /// pop entry from stack
    /// return popped entry
    pub fn pop_entry(&mut self) -> Result<PathBuf> {
        let entry = self.stack.pop();
        self.write_stack_file()?;

        match entry {
            Some(entry) => Ok(entry),
            None => Err(Error::other(
                "-- pop failed to retrieve item from stack, it might be empty",
            )),
        }
    }

    /// get entry by number without removing it from the stack
    /// return nth last entry
    pub fn get_entry_by_number(&mut self, entry_number: usize) -> Result<&PathBuf> {
        // index from the end of the vector as new entries are appended at the end of the list
        match self.stack.get(
            self.stack
                .len()
                .checked_sub(entry_number)
                .expect("-- requested entry number is out of bounds"),
        ) {
            Some(item) => Ok(item),
            None => Err(Error::other(
                "-- failed to retrieve stack entry by number",
            )),
        }
    }

    /// clean up dead stack files, parse and build stack
    fn build_stack(&mut self) -> Result<()> {
        let stack_dir: PathBuf = PathBuf::from_str("/tmp/navigation/")
            .expect("-- failed to create path object of '/tmp/navigation'");
        let mut sys = System::new_all();
        sys.refresh_all();
        let procs = sys.processes();

        if stack_dir.is_dir() {
            // clean up stack files of expired processes
            let members = fs::read_dir(stack_dir.clone())?;
            for entry in members {
                let entry = entry?;
                let process_id = Pid::from_str(
                    entry
                        .file_name()
                        .to_str()
                        .expect("-- failed to convert file name to str"),
                );
                if !procs.contains_key(&process_id.expect("-- failed to convert filename to pid")) {
                    fs::remove_file(entry.path()).expect("-- failed to remove orphaned file");
                }
            }
        } else {
            // create temporary directory to store the stack
            fs::create_dir(stack_dir.clone())?;
        }

        self.path = stack_dir.clone();
        self.path.push(PathBuf::from(&self.pid.to_string()));
        if self.path.is_file() {
            // read and parse stack file
            self.read_stack_file(&self.path.clone())?;
        } else {
            // create stack file and store current path
            File::create(self.path.clone())?;
        }

        Ok(())
    }

    /// parse stack file
    fn read_stack_file(&mut self, stack_file_path: &PathBuf) -> Result<()> {
        let stack = fs::read_to_string(stack_file_path)?;
        let stack = stack.split("\n");
        for entry in stack {
            self.stack.push(PathBuf::from(entry));
        }

        Ok(())
    }

    /// write stack current stack to file to save it for next execution
    fn write_stack_file(&mut self) -> Result<()> {
        let mut output = Vec::<&str>::new();
        for entry in &self.stack {
            output.push(
                entry
                    .to_str()
                    .expect("-- failed to convert stack entry to string"),
            );
        }
        fs::write(self.path.clone(), output.join("\n"))?;

        Ok(())
    }
} // end `impl database`
