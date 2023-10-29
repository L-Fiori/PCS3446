// In this file it is supposed to be implemented system
// abstractions such as memory, cpu and jobs.
use crate::event_list::{EventList, Metadata};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct Job {
    pub id: i32,
    pub state: i32,
    pub memory_size: i32,
    pub cpu_time: i32,
}

#[derive(Debug, Clone)]
pub struct Memory {
    pub available_memory: i32,
}

impl Memory {
    pub fn new(number: i32) -> Self {
        Memory { available_memory: number }
    }

    pub fn alloc(&mut self, num: i32) {
        // TODO: implement panic when memory is not available,
        // also further error treatments.
        self.available_memory = self.available_memory - num;
    }
}

pub struct SystemEntryQueue {
    jobs: Vec<Job>,
}

impl SystemEntryQueue {
    pub fn new() -> Self {
        SystemEntryQueue { jobs: Vec::new() }
    }

    pub fn add_job(&mut self, job: Job) {
        self.jobs.push(job);
    }

    pub fn remove_job(&mut self) -> Option<Job> {
        self.jobs.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.jobs.is_empty()
    }
}

pub struct MemoryAllocQueue {
    jobs: Vec<Job>,
}

impl MemoryAllocQueue {
    pub fn new() -> Self {
        MemoryAllocQueue { jobs: Vec::new() }
    }

    pub fn add_job(&mut self, job: Job) {
        self.jobs.push(job);
    }

    pub fn remove_job(&mut self) -> Option<Job> {
        self.jobs.pop()
    }
}

pub struct CPUAllocQueue {
    jobs: Vec<Job>,
}

impl CPUAllocQueue {
    pub fn new() -> Self {
        CPUAllocQueue { jobs: Vec::new() }
    }

    pub fn add_job(&mut self, job: Job) {
        self.jobs.push(job);
    }

    pub fn remove_job(&mut self) -> Option<Job> {
        self.jobs.pop()
    }
}

pub struct ExecQueue {
    jobs: Vec<Job>,
}

impl ExecQueue {
    pub fn new() -> Self {
        ExecQueue { jobs: Vec::new() }
    }

    pub fn add_job(&mut self, job: Job) {
        self.jobs.push(job);
    }

    pub fn remove_job(&mut self) -> Option<Job> {
        self.jobs.pop()
    }
    
    pub fn is_empty(&self) -> bool {
        self.jobs.is_empty()
    }
}

pub struct SharedState {
    event_list: Arc<Mutex<EventList>>,
    system_entry_queue: Arc<Mutex<SystemEntryQueue>>,
    memory_alloc_queue: Arc<Mutex<MemoryAllocQueue>>,
    cpu_alloc_queue: Arc<Mutex<CPUAllocQueue>>,
    exec_queue: Arc<Mutex<ExecQueue>>,
    memory: Arc<Mutex<Memory>>,
    pub current_timestep: i32,
}

impl SharedState {
    pub fn new(
        event_list: EventList,
        system_entry_queue: SystemEntryQueue,
        memory_alloc_queue: MemoryAllocQueue,
        cpu_alloc_queue: CPUAllocQueue,
        exec_queue: ExecQueue,
        memory: Memory,
        current_timestep: i32,
    ) -> Self {
        SharedState {
            event_list: Arc::new(Mutex::new(event_list)),
            system_entry_queue: Arc::new(Mutex::new(system_entry_queue)),
            memory_alloc_queue: Arc::new(Mutex::new(memory_alloc_queue)),
            cpu_alloc_queue: Arc::new(Mutex::new(cpu_alloc_queue)),
            exec_queue: Arc::new(Mutex::new(exec_queue)),
            memory: Arc::new(Mutex::new(memory)),
            current_timestep,
        }
    }

    pub fn get_event_list(&self) -> Arc<Mutex<EventList>> {
        self.event_list.clone()
    }

    pub fn get_system_entry_queue(&self) -> Arc<Mutex<SystemEntryQueue>> {
        self.system_entry_queue.clone()
    }

    pub fn get_memory_alloc_queue(&self) -> Arc<Mutex<MemoryAllocQueue>> {
        self.memory_alloc_queue.clone()
    }

    pub fn get_cpu_alloc_queue(&self) -> Arc<Mutex<CPUAllocQueue>> {
        self.cpu_alloc_queue.clone()
    }

    pub fn get_exec_queue(&self) -> Arc<Mutex<ExecQueue>> {
        self.exec_queue.clone()
    }

    pub fn get_memory(&self) -> Arc<Mutex<Memory>> {
        self.memory.clone()
    }
}

pub struct ControlModule {
    pub shared_state: SharedState,
}

impl ControlModule {
    pub fn new(shared_state: SharedState) -> Self {
        ControlModule { shared_state }
    }

    pub fn add_event(&self, time: i32, name: String, metadata: Metadata) {
        let event_list = self.shared_state.get_event_list();
        let mut list = event_list.lock().unwrap();
        list.push(time, name, metadata);
    }

    pub fn add_SEQ(&self, job: Job) {
        let system_entry_queue = self.shared_state.get_system_entry_queue();
        let mut queue = system_entry_queue.lock().unwrap();
        queue.add_job(job);
    }

    pub fn remove_SEQ(&self) {
        let system_entry_queue = self.shared_state.get_system_entry_queue();
        let mut queue = system_entry_queue.lock().unwrap();
        queue.remove_job();
    }

    pub fn add_MAQ(&self, job: Job) {
        let memory_alloc_queue = self.shared_state.get_memory_alloc_queue();
        let mut queue = memory_alloc_queue.lock().unwrap();
        queue.add_job(job);
    }
    
    pub fn remove_MAQ(&self) {
        let memory_alloc_queue = self.shared_state.get_memory_alloc_queue();
        let mut queue = memory_alloc_queue.lock().unwrap();
        queue.remove_job();
    }

    pub fn add_CAQ(&self, job: Job) {
        let cpu_alloc_queue = self.shared_state.get_cpu_alloc_queue();
        let mut queue = cpu_alloc_queue.lock().unwrap();
        queue.add_job(job);
    }

    pub fn remove_CAQ(&self) {
        let cpu_alloc_queue = self.shared_state.get_cpu_alloc_queue();
        let mut queue = cpu_alloc_queue.lock().unwrap();
        queue.remove_job();
    }

    pub fn add_EQ(&self, job: Job) {
        let exec_queue = self.shared_state.get_exec_queue();
        let mut queue = exec_queue.lock().unwrap();
        queue.add_job(job);
    }

    pub fn remove_EQ(&self) {
        let exec_queue = self.shared_state.get_exec_queue();
        let mut queue = exec_queue.lock().unwrap();
        queue.remove_job();
    }

    pub fn eq_is_empty(&self) -> bool {
       let exec_queue = self.shared_state.get_exec_queue();
       let queue = exec_queue.lock().unwrap();
       queue.is_empty()
    }

    pub fn alloc_memory(&self, num: i32) {
        let memory = self.shared_state.get_memory();
        let mut mem = memory.lock().unwrap();
        println!("Available memory left: {}k", mem.available_memory);
        mem.alloc(num);
        println!("Allocated {}k memory for the job. {}k memory space remaining.", num, mem.available_memory);
    }

    pub fn get_current_timestep(&self) -> i32 {
        let current_timestep = self.shared_state.current_timestep;
        current_timestep
    }

    pub fn update_current_timestep(&mut self, current_timestep: i32) {
        self.shared_state.current_timestep = current_timestep;
    }

    pub fn seq_is_empty(&self) -> bool {
        let system_entry_queue = self.shared_state.get_system_entry_queue();
        let queue = system_entry_queue.lock().unwrap();
        queue.is_empty()
    }
}
