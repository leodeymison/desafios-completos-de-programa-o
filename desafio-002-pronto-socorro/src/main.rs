use uuid::Uuid;

enum Classifications { VERMELHO, LARANJA, AMARELO, VERDE, AZUL }

enum Specialties { CLINICO_GERAL, PEDIATRIA, ORTOPEDIA, CARDIOLOGIA }

pub struct Time {
    classification: Classifications,
    time: i16
}

static TIME_START: &str = "07:00";
static TIME_END: &str = "19:00";

static TIMES: [Time; 5] = [
    Time { classification: Classifications::VERMELHO, time: 0 },
    Time { classification: Classifications::LARANJA, time: 10 },
    Time { classification: Classifications::AMARELO, time: 60 },
    Time { classification: Classifications::VERDE, time: 120 },
    Time { classification: Classifications::AZUL, time: 240 },
];

pub struct Patient {
    pub _id: Uuid,
    pub name: &'static str,
    pub classification: Classifications,
    pub specialty: Specialties
}

struct Triage {}

impl Triage {
    pub fn new() -> Triage {
        Triage {}
    }
}

fn main() {
    println!("Hello, world!");
}
