
use std::io::{Result};
use std::fs;
use std::fs::File;
use std::path::{PathBuf};
use std::str::FromStr;
use sysinfo::{System, Pid};


#[derive(Debug, Clone)]
pub struct Stack
{
    pid: u32,
    stack: Vec<PathBuf>,
}

impl Stack
{

    pub fn new(process_id: u32) -> Result<Self> {
        let mut stack: Stack = Stack{
            pid : process_id,
            stack : Vec::<PathBuf>::new(),
        };

        stack.stack = stack.build_stack()?;

        return Ok(stack);
    }

    /**
     * push entry to stack
     * returns updated stack
     */
    pub fn push_entry(&mut self, path: PathBuf) -> Vec<PathBuf> {
        self.stack.push(path);
        return self.stack.clone();
    }

    /**
     * pop entry from stack
     * return poppe entry
     */
    pub fn pop_entry(&mut self) -> PathBuf {
        return self.stack.pop().unwrap();
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
    
        let mut stack_file_path = stack_dir.clone();
        stack_file_path.push(PathBuf::from(&self.pid.to_string()));
        if stack_file_path.is_file() {
            // read and parse stack file
            let _ = self.parse_stack_file(stack_file_path);
        } else {
            // create stack file and store current path
            let _ = File::create(stack_file_path.clone())?;
        }
    
        return Ok(vec![stack_dir]);
    }

    /**
     * parse stack file
     */
    fn parse_stack_file(&mut self, stack_file_path: PathBuf) -> Result<()> {
        let stack = fs::read_to_string(stack_file_path)?;
        let stack = stack.split("\n");
        for entry in stack {
            self.stack.push(PathBuf::from(entry));
        }

        return Ok(());
    }

} // end `impl database`

