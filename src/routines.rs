use std::collections::HashMap;
use crate::system_abstractions::{Job, ControlModule};
use crate::event_list::{Metadata};

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
        "EndProcess" => Box::new(EndProcess{metadata: metadata.clone()}),
        "FreeCPU" => Box::new(FreeCPU{metadata: metadata.clone()}),
        "FreeMemory" => Box::new(FreeMemory{metadata: metadata.clone()}),
        "ExitSystem" => Box::new(ExitSystem{metadata: metadata.clone()}),
        "PauseJob" => Box::new(PauseJob{metadata: metadata.clone()}),
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
    event_to_routine.insert("Pause job", "PauseJob");
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
        println!("JobArrival esta rodando!");

        // Add the new job to the system entry queue
        let job_number = self.unwrap_metadata().0;
        let job_memory_size = self.unwrap_metadata().1;
        let job_cpu_time = self.unwrap_metadata().2;
        let new_job = Job {id: job_number, state: 1, memory_size: job_memory_size, cpu_time: job_cpu_time};
        
        if control_module.eq_is_empty() {

            // Add the job entrance event to be immediately treated

            control_module.add_event(0, "Ingresso de job".to_string(), Metadata::JobEntrance(new_job));

        } else {
            control_module.add_SEQ(new_job.clone());
        }

        println!("JobArrival terminou!");
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
        println!("JobEntrance esta rodando!");

        if let Some(mut job) = self.unwrap_metadata() {

            control_module.remove_SEQ();

            job.state = 2;

            // Add the request memory event to be immediately treated

            control_module.add_event(0, "Requisicao de memoria de job".to_string(), Metadata::RequestMemory(job.clone()));
        }
        println!("JobEntrance terminou!");
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
        println!("RequestMemory esta rodando!");
        println!("\n");
        // Verifica inicialmente se há algum job na fila de
        // alocação de memória. Se não ocorrer, e houver área livre,
        // alocam-se para o job X a quantidade de memória solicitada,
        // e atualiza a quantidade de memória restante. Supostamente
        // o loader deve carregar o código do Job na área alocada e
        // o job passa para estado 3 (pronto para execução) e passa
        // a aguardar na fila do processador. A seguir, é inserido o
        // evento dependente “Requisição de Processador Job X” para
        // tratamento imediato.
    
        if let Some(mut job) = self.unwrap_metadata() {
            let num = job.memory_size;
            let result = control_module.alloc_memory(job.clone(), num);
            match result {
                Ok(value) => {
                    job.state = 3;

                    // Add the request cpu event to be immediately treated

                    control_module.add_event(0, "Requisicao de processador de job".to_string(), Metadata::RequestCPU(job.clone()));
                }
                Err(value) => {
                    control_module.add_MAQ(job);
                    println!("Job adicionado a fila de alocacao de memoria. O sistema tentara alocar a memoria novamente apos a saida de algum job do sistema.");
                    let new_job = control_module.remove_CAQ().unwrap();
                    control_module.add_event(0, "Requisicao de processador de job".to_string(), Metadata::RequestCPU(new_job.clone()));
                }
            }
        }
        println!("RequestMemory terminou!");
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
        println!("RequestCPU esta rodando!");
        println!("\n");
        // Insere o job X na fila de execução, para ser
        // devidamente processado (estado 4). Agora, o
        // job X passou a ser executado. Daí, insere-se o
        // evento “Fim de processamento Job X”, calculando o
        // instante de término do processamento do job com
        // base no instante corrente e o tempo de execução
        // previsto para o job X.
        
        let time_slice = 10;
        let current_timestep = control_module.get_current_timestep();
        
        if let Some(mut job) = self.unwrap_metadata() {    
            job.state = 4;
            if !control_module.job_exists_in_table(job.id) {
                let job_cpu_time = job.cpu_time;
                
                control_module.add_to_job_table(job.id, job_cpu_time);

                println!("Timestep atual: {}", current_timestep);
                println!("Tempo de cpu do job: {}", job_cpu_time);

                let state_end = current_timestep + time_slice;
                println!("Fim do uso da cpu: {}", state_end);
                println!("\n");
                control_module.add_EQ(job.clone());

                // Add the PauseJob event to be treated after job_cpu_time
                // timesteps.

                if job_cpu_time > time_slice {
                    control_module.add_event(state_end, "Pause job".to_string(), Metadata::PauseJob(job));
                } else {
                    control_module.add_event(state_end, "Fim de processamento de job".to_string(), Metadata::EndProcess(job));
                }
            } else {
                // dai significa que estamos pedindo cpu de novo
                // apos o job ja ter executado por um timeslice
               
                let time_remaining = control_module.get_time_remaining(job.id);
                println!("Processing time remaining for job {}: {}", job.id, time_remaining);
                control_module.add_EQ(job.clone());
                if time_remaining <= time_slice {
                    let state_end = current_timestep + time_remaining;
                    control_module.add_event(state_end, "Fim de processamento de job".to_string(), Metadata::EndProcess(job));
                } else {
                    let state_end = current_timestep + time_slice;
                    control_module.add_event(state_end, "Pause job".to_string(), Metadata::PauseJob(job));
                }
            }
            println!("EventList: {:?}", control_module.shared_state.get_event_list());
        }
        println!("\n");
        println!("RequestCPU terminou!");
    }
}


struct PauseJob {
    metadata: Metadata,
}

impl PauseJob {
    fn unwrap_metadata(&self) -> Option<Job> {
        match &self.metadata {
            Metadata::PauseJob(Job) => Some(Job.clone()),
            _ => None,
        }
    }
}

impl Runnable for PauseJob {
    fn run(&self, control_module: &ControlModule) {
        println!("PauseJob esta rodando!");
        println!("\n");

        if let Some(mut job) = self.unwrap_metadata() {
            let time_slice = 10;
            control_module.update_job_table(job.id, time_slice);
            job.state = 3;

            // Checa se tem job na system entry queue e se o
            // numero de jobs atualmente rodando eh menor do que
            // um certo numero
            
            let max_jobs = 2;
            if !control_module.table_is_full(max_jobs) && !control_module.seq_is_empty() {

                let mut old_job = control_module.remove_EQ().unwrap();
                println!("Removido job {} da fila de execucao", old_job.id);
                control_module.add_CAQ(old_job);

                println!("Fila de Entrada no Sistema: {:?}", control_module.shared_state.get_system_entry_queue());
                let mut new_job = control_module.remove_SEQ().unwrap();

                new_job.state = 2;
                
                // Add the request memory event to be immediately treated

                control_module.add_event(0, "Requisicao de memoria de job".to_string(), Metadata::RequestMemory(new_job.clone()));
            } else {
                // Manda mais um requestCPU pro proximo job da fila de cpu,
                // podendo evidentemente ser o mesmo job

                println!("Fila de execucao: {:?}", control_module.shared_state.get_exec_queue());
                let mut new_job = control_module.remove_EQ().unwrap();
                println!("Fila de execucao apos remocao: {:?}", control_module.shared_state.get_exec_queue());
                println!("\n");
                control_module.add_CAQ(new_job);

                println!("Fila de Alocacao de Processador: {:?}", control_module.shared_state.get_cpu_alloc_queue());
                let actual_new_job = control_module.remove_CAQ().unwrap();
                println!("Fila de Alocacao de Processador apos remocao: {:?}", control_module.shared_state.get_cpu_alloc_queue());

                control_module.add_event(0, "Requisicao de processador de job".to_string(), Metadata::RequestCPU(actual_new_job));

            }
        }
        println!("\n");
        println!("PauseJob terminou!");
    }
}


struct EndProcess {
    metadata: Metadata,
}

impl EndProcess {
    fn unwrap_metadata(&self) -> Option<Job> {
        match &self.metadata {
            Metadata::EndProcess(Job) => Some(Job.clone()),
            _ => None,
        }
    }
}

impl Runnable for EndProcess {
    fn run(&self, control_module: &ControlModule) {
        println!("EndProcess esta rodando!");
        // se houver um job na fila de ingresso ao sistema,
        // ele deve ser retirado dessa fila. O tratamento
        // consiste em realizar três atividades de tratamento
        // imediato: a liberação do processador, a liberação
        // da memória e a saída do sistema. Inicialmente insere-se,
        // na fila de eventos, para o Job a ser terminado, o
        // evento dependente de liberação de processador.

        if let Some(job) = self.unwrap_metadata() {
            control_module.delete_job_table(job.id);
            control_module.remove_EQ();
            control_module.add_event(0, "Liberacao de processador job".to_string(), Metadata::FreeCPU(job));
        }
        println!("EndProcess terminou!");
    }
}

struct FreeCPU {
    metadata: Metadata,
}

impl FreeCPU {
    fn unwrap_metadata(&self) -> Option<Job> {
        match &self.metadata {
            Metadata::FreeCPU(Job) => Some(Job.clone()),
            _ => None,
        }
    }
}

impl Runnable for FreeCPU {
    fn run(&self, control_module: &ControlModule) {
        println!("FreeCPU esta rodando!");

        if let Some(mut job) = self.unwrap_metadata() {
            job.state = 5;
            control_module.remove_EQ();
            control_module.add_event(0, "Liberacao de memoria job".to_string(), Metadata::FreeMemory(job));
        }
        println!("FreeCPU terminou!");
    }
}

struct FreeMemory {
    metadata: Metadata,
}

impl FreeMemory {
    fn unwrap_metadata(&self) -> Option<Job> {
        match &self.metadata {
            Metadata::FreeMemory(Job) => Some(Job.clone()),
            _ => None,
        }
    }
}

impl Runnable for FreeMemory {
    fn run(&self, control_module: &ControlModule) {
        println!("FreeMemory esta rodando!");

        if let Some(mut job) = self.unwrap_metadata() {
            job.state = 6;
            let num = job.memory_size;
            control_module.dealloc_memory(job.clone());
            control_module.add_event(0, "Saida do sistema job".to_string(), Metadata::ExitSystem(job));
        }
        println!("FreeMemory terminou!");
    }
}

struct ExitSystem {
    metadata: Metadata,
}

impl ExitSystem {
    fn unwrap_metadata(&self) -> Option<Job> {
        match &self.metadata {
            Metadata::ExitSystem(Job) => Some(Job.clone()),
            _ => None,
        }
    }
}

impl Runnable for ExitSystem {
    fn run(&self, control_module: &ControlModule) {
        println!("ExitSystem esta rodando!");

        if !control_module.maq_is_empty() {
            println!("Fila de alocacao de memoria contem algum job: inserindo evento dependente de requisicao de memoria ao sistema.");

            let mut job = control_module.remove_MAQ().unwrap();

            // Add the request memory event to be immediately treated

            control_module.add_event(0, "Requisicao de memoria de job".to_string(), Metadata::RequestMemory(job.clone()));
        } else if !control_module.seq_is_empty() {
            println!("Fila de ingresso ao sistema contem algum evento: inserindo evento dependente de requisicao de memoria ao sistema.");

            let mut job = control_module.remove_SEQ().unwrap();

            job.state = 2;

            // Add the request memory event to be immediately treated

            control_module.add_event(0, "Requisicao de memoria de job".to_string(), Metadata::RequestMemory(job.clone()));
        } else {
            println!("Fila de ingresso ao sistema nao contem nenhum evento e fila de alocacao de memoria nao contem nenhum job.")
        }
        println!("ExitSystem terminou!");
    }
}

