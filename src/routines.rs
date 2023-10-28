use std::collections::HashMap;
use crate::system_abstractions::{Job, ControlModule};
use crate::event_list::{Metadata, Event};

pub fn select_routine<'a>(event_to_routine: &'a HashMap<&'a str, &'a str>, event_name: &'a str) -> &'a str {
    match event_to_routine.get(event_name) {
        Some(&routine) => routine,
        None => "DefaultRoutine",
    }
}

pub trait Runnable {
    fn run(&self, control_module: &ControlModule);
}

pub fn create_routine(routine: &str, metadata: &Metadata) -> Box<dyn Runnable> {
    match routine {
        "JobArrival" => Box::new(JobArrival{metadata: metadata.clone()}),
        "JobEntrance" => Box::new(JobEntrance{metadata: metadata.clone()}),
        "RequestMemory" => Box::new(RequestMemory{metadata: metadata.clone()}),
        "RequestCPU" => Box::new(RequestCPU{metadata: metadata.clone()}),
        "EndProcess" => Box::new(EndProcess),
        "FreeCPU" => Box::new(FreeCPU),
        "FreeMemory" => Box::new(FreeMemory),
        "ExitSystem" => Box::new(ExitSystem),
        _ => Box::new(DefaultRoutine), // Handle unknown routines
    }
}

pub fn create_event_to_routine() -> HashMap<&'static str, &'static str> {
    // Create an empty HashMap
    let mut event_to_routine = HashMap::new();

    // Insert key-value pairs into the HashMap
    event_to_routine.insert("Chegada de job", "JobArrival");
    event_to_routine.insert("Ingresso de job", "JobEntrance");
    event_to_routine.insert("Requisicao de memoria de job", "RequestMemory");
    event_to_routine.insert("Requisicao de processador de job", "RequestCPU");
    event_to_routine.insert("Fim de processamento de job", "EndProcess");
    event_to_routine.insert("Liberacao de processador job", "FreeCPU");
    event_to_routine.insert("Liberacao de memoria job", "FreeMemory");
    event_to_routine.insert("Saida do sistema job", "ExitSystem");
    //event_to_routine.insert("", "");
    
    event_to_routine
}

//==================== ACTUAL ROUTINE IMPLS ====================

struct DefaultRoutine;
impl Runnable for DefaultRoutine {
    fn run(&self, control_module: &ControlModule) {
        println!("DefaultRoutine is running!");
    }
}

struct JobArrival{
    metadata: Metadata,
}

impl JobArrival {
    fn unwrap_metadata(&self) -> (i32, i32, i32) {
        match &self.metadata {
            Metadata::JobArrival(num, mem, cpu) => (*num, *mem, *cpu),
            _ => (0, 0, 0),
        }
    }
}

impl Runnable for JobArrival {
    fn run(&self, control_module: &ControlModule) {
        println!("JobArrival is running!");

        // Add the new job to the system entry queue
        let job_number = self.unwrap_metadata().0;
        let job_memory_size = self.unwrap_metadata().1;
        let job_cpu_time = self.unwrap_metadata().2;
        let new_job = Job {id: job_number, state: 1, memory_size: job_memory_size, cpu_time: job_cpu_time};
        control_module.add_SEQ(new_job.clone());

        // Add the job entrance event to be immediately treated

        let new_event = Box::new(Event {
            time: 0,
            name: "Ingresso de job".to_string(),
            metadata: Metadata::JobEntrance(new_job),
            next: None,
        });
        control_module.add_event(*new_event);

        // FALTA IMPLEMENTAR:
        // "Se houver um job em execução, isto é,
        // o processador estiver ocupado, o novo job
        // deverá aguardar na fila de espera pelo ingresso
        // ao sistema o término do job que está sendo executado."

        println!("JobArrival finished running!");
    }
}

struct JobEntrance {
    metadata: Metadata,
}

impl JobEntrance {
    fn unwrap_metadata(&self) -> Option<Job> {
        match &self.metadata {
            Metadata::JobEntrance(Job) => Some(Job.clone()),
            _ => None,
        }
    }
}

impl Runnable for JobEntrance {
    fn run(&self, control_module: &ControlModule) {
        println!("JobEntrance is running!");

        if let Some(mut job) = self.unwrap_metadata() {

            control_module.remove_SEQ();

            job.state = 2;

            // Add the request memory event to be immediately treated

            let new_event = Box::new(Event {
                time: 0,
                name: "Requisicao de memoria de job".to_string(),
                metadata: Metadata::RequestMemory(job.clone()),
                next: None,
            });
            control_module.add_event(*new_event);
        }
    }
}

struct RequestMemory {
    metadata: Metadata,
}

impl RequestMemory {
    fn unwrap_metadata(&self) -> Option<Job> {
        match &self.metadata {
            Metadata::RequestMemory(Job) => Some(Job.clone()),
            _ => None,
        }
    }
}

impl Runnable for RequestMemory {
    fn run(&self, control_module: &ControlModule) {
        println!("RequestMemory is running!");
        // Verifica inicialmente se há algum job na fila de
        // alocação de memória. Se não ocorrer, e houver área livre,
        // alocam-se para o job X a quantidade de memória solicitada,
        // e atualiza a quantidade de memória restante. Supostamente
        // o loader deve carregar o código do Job na área alocada e
        // o job passa para estado 3 (pronto para execução) e passa
        // a aguardar na fila do processador. A seguir, é inserido o
        // evento dependente “Requisição de Processador Job X” para
        // tratamento imediato.
        //
        // FALTA IMPLEMENTAR: interacoes om a fila de alocacao
        // de memoria, verificacao de area livre essas coisas.
    
        if let Some(mut job) = self.unwrap_metadata() {
            let num = job.memory_size;
            control_module.alloc_memory(num);
            job.state = 3;
            control_module.add_CAQ(job.clone());

            // Add the request memory event to be immediately treated

            let new_event = Box::new(Event {
                time: 0,
                name: "Requisicao de processador de job".to_string(),
                metadata: Metadata::RequestCPU(job.clone()),
                next: None,
            });
            control_module.add_event(*new_event);
        }
    }
}

struct RequestCPU {
    metadata: Metadata,
}

impl RequestCPU {
    fn unwrap_metadata(&self) -> Option<Job> {
        match &self.metadata {
            Metadata::RequestCPU(Job) => Some(Job.clone()),
            _ => None,
        }
    }
}

impl Runnable for RequestCPU {
    fn run(&self, control_module: &ControlModule) {
        println!("RequestCPU is running!");
        // Insere o job X na fila de execução, para ser
        // devidamente processado (estado 4). Agora, o
        // job X passou a ser executado. Daí, insere-se o
        // evento “Fim de processamento Job X”, calculando o
        // instante de término do processamento do job com
        // base no instante corrente e o tempo de execução
        // previsto para o job X.
        
        if let Some(mut job) = self.unwrap_metadata() {
            job.state = 4;
            let job_cpu_time = job.cpu_time;
            let current_timestep = control_module.get_current_timestep();
            println!("current timestep: {}", current_timestep);
            println!("job cpu time: {}", job_cpu_time);
            let state_end = current_timestep + job_cpu_time;
            println!("State end: {}", state_end);
            control_module.add_EQ(job);

            // Add the EndProcess event to be treated after job_cpu_time
            // timesteps.

            let new_event = Box::new(Event {
                time: state_end,
                name: "Fim de processamento de job".to_string(),
                metadata: Metadata::EndProcess,
                next: None,
            });
            control_module.add_event(*new_event);
            println!("EventList: {:?}", control_module.shared_state.get_event_list());

        }
    }
}

struct EndProcess;
impl Runnable for EndProcess {
    fn run(&self, control_module: &ControlModule) {
        println!("EndProcess is running!");
    }
}

struct FreeCPU;
impl Runnable for FreeCPU {
    fn run(&self, control_module: &ControlModule) {
        println!("FreeCPU is running!");
    }
}

struct FreeMemory;
impl Runnable for FreeMemory {
    fn run(&self, control_module: &ControlModule) {
        println!("FreeMemory is running!");
    }
}

struct ExitSystem;
impl Runnable for ExitSystem {
    fn run(&self, control_module: &ControlModule) {
        println!("ExitSystem is running!");
    }
}

