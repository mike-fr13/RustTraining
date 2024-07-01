#[derive(Debug)]
enum Ressenti {
    Froid,
    Doux,
    Chaud,
}

struct TemperatureMoyenne { 
    valeur: f32,
    categorie: Ressenti
}

fn main() {
    let temperatures : [f32; 7] = [22.0, 19.5, 21.0, 23.5, 20.0, 18.0, 25.0];
    let average = calculate_average(&temperatures);
    println!("Moyenne température : {:.2}°C", average);

    let res: Ressenti = calculate_ressenti(average);
    println!("res : {:?}", res);

    let temp_moyenne: TemperatureMoyenne = TemperatureMoyenne {
        valeur: average,
        categorie: res
    };
    
    match temp_moyenne.categorie {
        Ressenti::Froid => println!("Ressenti : il fait Froid : {:.2}°C", temp_moyenne.valeur),
        Ressenti::Doux => println!("Ressenti : il fait Doux : {:.2}°C", temp_moyenne.valeur),
        Ressenti::Chaud => println!("Ressenti : il fait Chaud : {:.2}°C", temp_moyenne.valeur),
    }




}

fn calculate_average(temperatures: &[f32]) -> f32 {
    let sum: f32 = temperatures.iter().sum();
    let count: f32 = temperatures.len() as f32;
    sum / count 
}

fn calculate_ressenti(temp: f32) -> Ressenti {
    match temp {
        x if x < 20.0 => 
            Ressenti::Froid,
        x if x >= 20.0 && x < 25.0 =>  
            Ressenti::Doux,
        _ =>
            Ressenti::Chaud,
    }
}


