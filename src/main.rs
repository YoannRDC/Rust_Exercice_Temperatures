
mod temperature;

use temperature::temp::{calculer_moyenne_temp, arrondir_a_deux_chiffres, obtenir_temperature_moyenne, trier_temperatures, categoriser_moyenne, Categorie, TemperatureMoyenne};
use std::io::{self, Write};

fn main() {

    // Instructions
    let mut temperatures: [f32;7] = [22.0,19.5,21.0,23.5,20.0,18.0,25.0];

    // Exo 1.
    let res1 = calculer_moyenne_temp(&temperatures).unwrap();
    println!("Exo 1: Moyenne: {}", res1);
    
    // Exo 2.
    let res2 = arrondir_a_deux_chiffres(res1);
    println!("Exo 2: Moyenne arrondie: {}", res2);

    // Exo 3.
    let res3 = categoriser_moyenne(res2);
    println!("Exo 3: Moyenne catégorisée: {:?}", res3);

    // Exo 4.
    let res4 = obtenir_temperature_moyenne(&temperatures);
    println!("Exo 4: Température moyenne via Objet: {:?}", res4);

    // Exo 5.
    let mut temperatures_vide: Vec<f32> = Vec::new();
    if temperatures_vide.is_empty() {
        println!("Exo 5: Aucune température");
    }

    // Exo 6.
    trier_temperatures(&mut temperatures);
    println!("Exo 6: Température moyenne via Objet: {:?}", temperatures);

    // Exo 7.
    match lire_temperatures() {
        Ok(mut temperatures) => {
            if temperatures.is_empty() {
                println!("Exo 7: Aucune température");
                return;
            }

            // Trier les températures
            trier_temperatures(&mut temperatures);
            println!("Exo 7: Températures triées: {:?}", temperatures);

            // Calculer la moyenne et obtenir la catégorie
            match obtenir_temperature_moyenne(&temperatures) {
                Some(result) => {
                    let categorie = match result.categorie {
                        Categorie::Froid => "Froid",
                        Categorie::Tempere => "Tempéré",
                        Categorie::Chaud => "Chaud",
                    };
                    println!("Exo 7: Moyenne température: {:.2}°C, Catégorie: {}", result.moyenne, categorie);
                }
                None => println!("Exo 7: Aucune température"),
            }
        }
        Err(err) => println!("Exo 7: {}", err),
    }

}

fn lire_temperatures() -> Result<Vec<f32>, String> {
    let mut temperatures = Vec::new();

    loop {
        print!("Exo 7: Entrez une température (ou 'fin' pour terminer) : ");
        io::stdout().flush().unwrap(); // Assurez-vous que l'invite est affichée immédiatement

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input.eq_ignore_ascii_case("fin") {
            break;
        }

        match input.parse::<f32>() {
            Ok(temp) => temperatures.push(temp),
            Err(_) => return Err(format!("Exo 7: Erreur : '{}' n'est pas un nombre valide", input)),
        }
    }

    Ok(temperatures)
}