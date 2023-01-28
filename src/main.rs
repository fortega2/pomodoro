use chrono::Duration;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let tiempo = args.get(1).expect("No se pudo calcular el tiempo").parse::<i64>().expect("No se puede tranformar una letra a número");
    
    let treinta_minutos_mas = chrono::Local::now()
        .checked_add_signed(Duration::minutes(tiempo))
        .expect("No se pudo calcular el dia")
        .format("%H%M%S")
        .to_string();

    loop {
        let time = chrono::Local::now().format("%H%M%S").to_string();

        if treinta_minutos_mas.get(..2).unwrap().parse::<isize>().unwrap() > time.get(..2).unwrap().parse::<isize>().unwrap() {
            let restante = (treinta_minutos_mas.parse::<isize>().unwrap() - time.parse::<isize>().unwrap()) / 100 - 40;
            if restante == 0 {
                println!("Se termino el tiempo");
                break;
            } else {
                println!("Minutos restantes: {}", restante);
            }
        } else {
            let restante = (treinta_minutos_mas.parse::<isize>().unwrap() - time.parse::<isize>().unwrap()) / 100;
            if restante == 0 {
                println!("Se termino el tiempo");
                break;
            } else {
                println!("Minutos restantes: {}", restante);
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}
