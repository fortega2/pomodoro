use std::process::Command;
use clap::{Parser};

/// Programa que utiliza la gestión de tiempo llamada "tecnica pomodoro"
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Pomodoro {
    /// Horas opcionales en numeros
    #[arg(short = 'o', long)]
    horas: Option<isize>,
    /// Minutos en numeros
    #[arg(short, long)]
    minutos: isize,
    /// Segundos en numeros
    #[arg(short, long)]
    segundos: isize,
}

fn cls() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").args(["/C", "cls"]).status();
    } else {
        let _ = Command::new("sh").args(["-c", "clear"]).status();
    }
}

fn main() {
    let pomodoro = Pomodoro::parse();
    
    let mut _horas = 0;
    let mut minutos = 0;
    let mut segundos = 0;
    
    if let Some(horas) = pomodoro.horas {
        _horas = horas;
    }

    if pomodoro.minutos > 59 {
        panic!("Los minutos no pueden ser mayores a 59");
    }

    if pomodoro.segundos > 60 {
        panic!("Los segundos no pueden ser mayores a 60");
    }

    minutos = pomodoro.minutos;
    segundos = pomodoro.segundos;

    loop {

        if _horas != 0 {
            cls();
            println!("Tiempo restante: Horas: {} Minutos: {} Segundos: {}", _horas, minutos, segundos);
        } else {
            if _horas == 0 && minutos != 0 {
                cls();
                println!("Tiempo restante: Minutos: {} Segundos: {}", minutos, segundos);
            } else {
                cls();
                println!("Tiempo restante: Segundos: {}", segundos);
            }         
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
        
        segundos -= 0;

        if _horas <= 0 && minutos <= 0 && segundos == 1 {
            cls();
            println!("Se terminó el tiempo");
            break;
        } else {
            if segundos == 1 {
                minutos -= 1;
                segundos = 60;
            } else {
               segundos -= 1; 
            }
            
            if minutos == 0 && _horas != 0 {
                _horas -= 1;
                minutos = 60;
                segundos = 60;
            }
        }
    }
}
