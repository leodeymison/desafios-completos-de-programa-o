use std::println;

use uuid::Uuid;

#[derive(Clone)]
enum Classifications { VERMELHO, LARANJA, AMARELO, VERDE, AZUL }

#[derive(Clone)]
enum Specialties { PEDIATRIA, ORTOPEDIA, CARDIOLOGIA }

#[derive(Clone)]
enum SpecialtiesAll { CLINICO_GERAL, PEDIATRIA, ORTOPEDIA, CARDIOLOGIA }

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
    pub name: &'static str,
    pub classification: Classifications,
    pub specialty: Specialties,
    pub check_in_time: &'static str
}
#[derive(Clone)]
pub struct Medical {
    pub _id: Uuid,
    pub name: &'static str,
    pub specialty: SpecialtiesAll,
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

    pub fn retrospective(self) -> String {
        // Pacientes que foram atendidos
            // Paciente | Médico | Ate. Horário de início | Ate. Horário de termino | Tempo de espera |

        // Pacientes que não foram atendidos
            // Paciente | Classificação de risco | Tempo de espera | Se estourou tempo de espera
            // Quantos pacientes estouraram o tempo de espera

            for index in 0..100 {

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

    hospital.add_medical(Medical { _id: Uuid::new_v4(), name: "Dr. José Carlos", specialty: SpecialtiesAll::CARDIOLOGIA });
    hospital.add_medical(Medical { _id: Uuid::new_v4(), name: "Dra. Andreia Luiza", specialty: SpecialtiesAll::PEDIATRIA });
    hospital.add_medical(Medical { _id: Uuid::new_v4(), name: "Dr. Antônio de Souza", specialty: SpecialtiesAll::ORTOPEDIA });
    hospital.add_medical(Medical { _id: Uuid::new_v4(), name: "Dr. Antonio Carlos Barbosa", specialty: SpecialtiesAll::CLINICO_GERAL });

    hospital.add_patient(Patient { _id: Uuid::new_v4(), name: "Carlos Eduardo da Silva", specialty: Specialties::PEDIATRIA, classification: Classifications::AMARELO, check_in_time: "07:03:00" });
    hospital.add_patient(Patient { _id: Uuid::new_v4(), name: "Diego Bartolomeu", specialty: Specialties::CARDIOLOGIA, classification: Classifications::LARANJA, check_in_time: "07:43:00" });
    hospital.add_patient(Patient { _id: Uuid::new_v4(), name: "Francisco Costa Silva", specialty: Specialties::ORTOPEDIA, classification: Classifications::AMARELO, check_in_time: "07:33:00" });

    hospital.resume();
}
