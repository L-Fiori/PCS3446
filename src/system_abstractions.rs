// In this file it is supposed to be implemented system
// abstractions such as memory, cpu and jobs.
use crate::event_list::{EventList, Metadata};
use std::sync::{Arc, Mutex};
use std::collections::{HashMap};

#[derive(Clone, Debug, PartialEq)]
pub struct Job {
    pub id: i32,
    pub state: i32,
    pub memory_size: i32,
    pub cpu_time: i32,
}

pub struct JobTable {
    table: HashMap<i32, i32>,
}

impl JobTable {
    pub fn new() -> Self {
        JobTable { table: HashMap::new() }
    }

    fn add_job(&mut self, job_id: i32, execution_time: i32) {
        self.table.insert(job_id, execution_time);
    }

    fn pause_job(&mut self, job_id: i32, time_slice: i32) {
        if let Some(remaining_time) = self.table.get_mut(&job_id) {
            *remaining_time -= time_slice;
        }
    }

    fn get_time_remaining(&mut self, job_id: i32) -> i32 {
        if let Some(remaining_time) = self.table.get_mut(&job_id) {
            *remaining_time
        } else {
            -1
        }
    }
}

#[derive(Debug, Clone)]
pub struct Segment {
    id: i32,
    start_address: i32,
    size: i32,
    owner: Option<Job>,
}

#[derive(Debug, Clone)]
pub struct Memory {
    total_memory: i32,
    next_segment_id: i32,
    segments: Vec<Segment>,
}

impl Memory {
    pub fn new(number: i32) -> Self {
        Memory {
            total_memory: number,
            next_segment_id: 1,
            segments: Vec::new(),
        }
    }

    pub fn alloc(&mut self, job: Job, size: i32) -> Result<Segment, &'static str> {
        let segment = self.allocate_segment(size);
        if let Some(segment) = segment {
            self.segments.push(Segment {
                id: self.next_segment_id,
                start_address: segment.start_address,
                size,
                owner: Some(job.clone()),
            });
            self.next_segment_id += 1;
            println!(
                "Segmento alocado: ID={}, Endereco de inicio={}, Tamanho={} para o Job {}",
                segment.id,
                segment.start_address,
                segment.size,
                job.id
            );
            Ok(segment)
        } else {
            Err("Alocacao de memoria falhou: Sem espaco disponivel na memoria")
        }
    }

    pub fn dealloc(&mut self, job: Job) {
        let indices: Vec<usize> = self
            .segments
            .iter()
            .enumerate()
            .filter_map(|(index, segment)| {
                if segment.owner.as_ref().map_or(false, |owner| owner.id == job.id) {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();

        for &index in indices.iter().rev() {
            let segment = self.segments.remove(index);
            // Deallocate the memory used by the segment
            println!(
                "Segmento desalocado: ID={}, Endereco de inicio={}, Tamanho={} (relativo ao Job {})",
                segment.id,
                segment.start_address,
                segment.size,
                job.id
            ); 
        }
    }

    pub fn available_memory(&self) -> i32 {
        self.total_memory - self.segments.iter().map(|s| s.size).sum::<i32>()
    }

    fn allocate_segment(&mut self, size: i32) -> Option<Segment> {
        let mut start_address = 0;

        for segment in &self.segments {
            let gap_size = segment.start_address - start_address;
            if gap_size >= size {
                // Found a suitable gap
                return Some(Segment {
                    id: 0, // You can set the correct ID when inserting into the segments vector
                    start_address,
                    size,
                    owner: None,
                });
            }
            start_address = segment.end_address();
        }

        // Check for available memory after the last segment
        let remaining_memory = self.total_memory - start_address;
        if remaining_memory >= size {
            return Some(Segment {
                id: 0, // You can set the correct ID when inserting into the segments vector
                start_address,
                size,
                owner: None,
            });
        }

        // If no suitable gap is found, return None
        None
    }
}

impl Segment {
    fn end_address(&self) -> i32 {
        self.start_address + self.size
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
    job_table: Arc<Mutex<JobTable>>,
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
        job_table: JobTable,
    ) -> Self {
        SharedState {
            event_list: Arc::new(Mutex::new(event_list)),
            system_entry_queue: Arc::new(Mutex::new(system_entry_queue)),
            memory_alloc_queue: Arc::new(Mutex::new(memory_alloc_queue)),
            cpu_alloc_queue: Arc::new(Mutex::new(cpu_alloc_queue)),
            exec_queue: Arc::new(Mutex::new(exec_queue)),
            memory: Arc::new(Mutex::new(memory)),
            current_timestep,
            job_table: Arc::new(Mutex::new(job_table)),
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
    
    pub fn get_job_table(&self) -> Arc<Mutex<JobTable>> {
        self.job_table.clone()
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

    pub fn remove_SEQ(&self) -> Option<Job> {
        let system_entry_queue = self.shared_state.get_system_entry_queue();
        let mut queue = system_entry_queue.lock().unwrap();
        queue.remove_job()
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

    pub fn alloc_memory(&self, job: Job, num: i32) -> Result<Segment, &'static str> {
        let memory = self.shared_state.get_memory();
        let mut mem = memory.lock().unwrap();
        println!("Memoria livre restante: {}k", mem.available_memory());
        let result = mem.alloc(job.clone(), num);
        match result {
            Ok(_) => println!(""),
            Err(error) => println!("Memory allocation failed: {}", error),
        }
        result
    }

    pub fn dealloc_memory(&self, job: Job) {
        let memory = self.shared_state.get_memory();
        let mut mem = memory.lock().unwrap();
        println!("Memoria livre disponivel: {}k", mem.available_memory());
        mem.dealloc(job.clone());
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

    pub fn add_to_job_table(&self, id: i32, cpu_time: i32) {
        let job_table = self.shared_state.get_job_table();
        let mut table = job_table.lock().unwrap();
        table.add_job(id, cpu_time);
    }

    pub fn update_job_table(&mut self, id: i32, time_slice: i32) {
        let job_table = self.shared_state.get_job_table();
        let mut table = job_table.lock().unwrap();
        table.pause_job(id, time_slice);
    }

    pub fn get_time_remaining(&self, id: i32) -> i32 {
        let job_table = self.shared_state.get_job_table();
        let mut table = job_table.lock().unwrap();
        table.get_time_remaining(id)
    }
}
