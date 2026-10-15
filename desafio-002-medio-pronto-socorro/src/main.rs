use std::println;

use uuid::Uuid;

#[derive(Clone)]
enum Classifications { VERMELHO, LARANJA, AMARELO, VERDE, AZUL }

#[derive(Clone, PartialEq, Debug)]
enum SpecialtiesAll { CLINICO_GERAL, PEDIATRIA, ORTOPEDIA, CARDIOLOGIA }

#[derive(Clone)]
enum Status { ESPERANDO, EM_ATENDIMENTO, ATENDIDO, DESISTIU }

#[derive(Clone, PartialEq, Debug)]
enum StatusMedical { COM_PACIENTE, SEM_PACIENTE }


pub struct Time {
    classification: Classifications,
    time: i16
}

static TIMES: [Time; 5] = [
    Time { classification: Classifications::VERMELHO, time: 0 },
    Time { classification: Classifications::LARANJA, time: 10 },
    Time { classification: Classifications::AMARELO, time: 60 },
    Time { classification: Classifications::VERDE, time: 120 },
    Time { classification: Classifications::AZUL, time: 240 },
];
#[derive(Clone)]
pub struct Patient {
    pub _id: Uuid,
    pub name: String,
    pub classification: Classifications,
    pub specialty: SpecialtiesAll,
    pub check_in_time: String,
    pub check_out_time: Option<String>,
    pub status: Status
}
#[derive(Clone)]
pub struct Medical {
    pub _id: Uuid,
    pub name: &'static str,
    pub specialty: SpecialtiesAll,
    pub status: StatusMedical
}

#[derive(Clone)]
struct Triage {
    patients: Vec<Patient>,
    medicals: Vec<Medical>,
    time_start: &'static str,
    time_end: &'static str,
}

impl Triage {
    pub fn new(time_start: &'static str, time_end: &'static str) -> Triage {
        Triage { time_start, time_end, patients: vec![], medicals: vec![] }
    }

    pub fn add_patient(&mut self, patient: Patient) {
        self.patients.push(patient);
    }

    pub fn add_medical(&mut self, medical: Medical) {
        self.medicals.push(medical);
    }

    fn get_working_time(time_start: &'static str, time_end: &'static str) -> i32 {
        return 32;
    }

    fn get_check_in_time(check_in_time: &'static str) -> i32 {
        return 32;
    }

    fn next_patient(patients: &Vec<Patient>, medical: &Medical) {
        if (medical.specialty != SpecialtiesAll::CLINICO_GERAL) {
            // ? Pega pacientes da especialidade do mético
            let patients_specialty = patients.iter().filter(|patient| patient.specialty == medical.specialty);

            // ? Filtra pelas cores

            // ? Pega que chegou primeiro
        }
    }

    pub fn retrospective(self) -> String {
        // Pacientes que foram atendidos
            // Paciente | Médico | Ate. Horário de início | Ate. Horário de termino | Tempo de espera |

        // Pacientes que não foram atendidos
            // Paciente | Classificação de risco | Tempo de espera | Se estourou tempo de espera
            // Quantos pacientes estouraram o tempo de espera

        let working_time = Self::get_working_time(self.time_start, self.time_end);

        for index in 0..working_time {
            for medical in &self.medicals {
                // ? Adiciona paciente para médicos que estão sem pacientes
                if medical.status == StatusMedical::SEM_PACIENTE {
                    let next_patient = Self::next_patient(&self.patients, &medical);
                }
            }
        }

        return String::from("");
    }

    pub fn resume(self) {
        println!("{}", "=".repeat(28));
        println!("{} Resumo {}", "=".repeat(10), "=".repeat(10));
        println!("{}", "=".repeat(28));
        println!("+ Médicos: {}", self.medicals.len());
        println!("+ Paciêntes: {}", self.patients.len());
        println!("{}", "=".repeat(28));

        let retrospective = self.retrospective();
    }
}

fn main() {
    let mut hospital = Triage::new("07:00", "19:00");

    hospital.add_medical(Medical { _id: Uuid::new_v4(), name: "Dr. José Carlos", specialty: SpecialtiesAll::CARDIOLOGIA, status: StatusMedical::SEM_PACIENTE });
    hospital.add_medical(Medical { _id: Uuid::new_v4(), name: "Dra. Andreia Luiza", specialty: SpecialtiesAll::PEDIATRIA, status: StatusMedical::SEM_PACIENTE });
    hospital.add_medical(Medical { _id: Uuid::new_v4(), name: "Dr. Antônio de Souza", specialty: SpecialtiesAll::ORTOPEDIA, status: StatusMedical::SEM_PACIENTE });
    hospital.add_medical(Medical { _id: Uuid::new_v4(), name: "Dr. Antonio Carlos Barbosa", specialty: SpecialtiesAll::CLINICO_GERAL, status: StatusMedical::SEM_PACIENTE });

    hospital.add_patient(Patient { _id: Uuid::new_v4(), name: "Carlos Eduardo da Silva".to_string(), specialty: SpecialtiesAll::PEDIATRIA, classification: Classifications::AMARELO, check_in_time: "07:03:00".to_string(), status: Status::ESPERANDO, check_out_time: None });
    hospital.add_patient(Patient { _id: Uuid::new_v4(), name: "Diego Bartolomeu".to_string(), specialty: SpecialtiesAll::CARDIOLOGIA, classification: Classifications::LARANJA, check_in_time: "07:43:00".to_string(), status: Status::ESPERANDO, check_out_time: None });
    hospital.add_patient(Patient { _id: Uuid::new_v4(), name: "Francisco Costa Silva".to_string(), specialty: SpecialtiesAll::ORTOPEDIA, classification: Classifications::AMARELO, check_in_time: "07:33:00".to_string(), status: Status::ESPERANDO, check_out_time: None });

    hospital.resume();
}
