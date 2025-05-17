//programa que calcula horas para segundos

const SECONDS_IN_MINUTES: u32 = 60;
const MINUTES_IN_HOURS: u32 = 60;
const SECONDS_IN_HOURS: u32 = SECONDS_IN_MINUTES * MINUTES_IN_HOURS;

fn main() {
    let total = 30;
    println!("Trabalhador trabalhou {} horas", total);
    let total_em_segundos = total * SECONDS_IN_HOURS;
    println!("Trabalhador trabalhou {} segundos", total_em_segundos);
}
