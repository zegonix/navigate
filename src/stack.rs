
use std::fs;
use std::fs::File;
use std::path::{PathBuf};
use std::str::FromStr;
use std::io::{Result, Error, ErrorKind};
use sysinfo::{System, Pid};


#[derive(Debug, Clone)]
pub struct Stack
{
    pid: u32,
    path: PathBuf,
    stack: Vec<PathBuf>,
}

impl Stack
{

    pub fn new(process_id: u32) -> Result<Self> {
        let mut stack: Stack = Stack{
            pid : process_id,
            path : PathBuf::new(),
            stack : Vec::<PathBuf>::new(),
        };

        _ = stack.build_stack()?;

        return Ok(stack);
    }

    /**
     * return stack
     */
    pub fn get_stack(&mut self) -> Result<&Vec<PathBuf>> {
        return Ok(&self.stack);
    }

    /**
     * push entry to stack
     * returns updated stack
     */
    pub fn push_entry(&mut self, path: &PathBuf) -> Result<&Vec<PathBuf>> {
        let abs_path = path.canonicalize()?;
        self.stack.push(abs_path);
        self.write_stack_file()?;
        return Ok(&self.stack);
    }

    /**
     * pop entry from stack
     * return poppe entry
     */
    pub fn pop_entry(&mut self) -> Result<PathBuf> {
        let entry = self.stack.pop();
        self.write_stack_file()?;

        return match entry {
            Some(entry) => Ok(entry),
            None => Err(Error::new(ErrorKind::Other, "pop failed to retrieve item from stack")),
        }
    }

    /**
     * get entry by number
     * return nth last entry
     */
    pub fn get_entry_by_number(&mut self, entry_number: u32) -> Result<&PathBuf> {
        if entry_number > (self.stack.len() as u32 - 1) {
            return Err(Error::new(ErrorKind::Other, "requested entry number is out of bounds"));
        }
        // index from the end of the vector as new entries are appended at the end of the list
        return Ok(&self.stack[self.stack.len() - (entry_number as usize)]);
    }

    /**
     * clean up dead stack files, parse and build stack
     */
    fn build_stack(&mut self) -> Result< Vec<PathBuf> > {
        let stack_dir: PathBuf = PathBuf::from_str("/tmp/navigation/").expect("failed to create path object of '/tmp/navigation'");
        let mut sys = System::new_all();
        sys.refresh_all();
        let procs = sys.processes();
    
        if stack_dir.is_dir() {
            // clean up stack files of expired processes
            let members = fs::read_dir(stack_dir.clone())?;
            for entry in members {
                let entry = entry?;
                let process_id = Pid::from_str(entry.file_name().to_str().expect("failed to convert file name to str"));
                if !procs.contains_key(&process_id.expect("failed to convert filename to pid")) {
                    fs::remove_file(entry.path()).expect("failed to remove orphaned file");
                }
            }
        } else {
            // create temporary directory to store the stack
            let _ = fs::create_dir(stack_dir.clone())?;
        }
    
        self.path = stack_dir.clone();
        self.path.push(PathBuf::from(&self.pid.to_string()));
        if self.path.is_file() {
            // read and parse stack file
            _ = self.read_stack_file(&self.path.clone())?;
        } else {
            // create stack file and store current path
            _ = File::create(self.path.clone())?;
        }
    
        return Ok(vec![stack_dir]);
    }

    /**
     * parse stack file
     */
    fn read_stack_file(&mut self, stack_file_path: &PathBuf) -> Result<()> {
        let stack = fs::read_to_string(stack_file_path)?;
        let stack = stack.split("\n");
        for entry in stack {
            self.stack.push(PathBuf::from(entry));
        }

        return Ok(());
    }

    /**
     * write stack current stack to file to save it for next execution
     */
    fn write_stack_file(&mut self) -> Result<()> {
        let mut output = Vec::<&str>::new();
        for entry in &self.stack {
            output.push(entry.to_str().expect("failed to convert stack entry to string"));
        }
        fs::write(self.path.clone(), output.join("\n"))?;

        return Ok(())
    }

} // end `impl database`

