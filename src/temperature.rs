pub mod temp {

    pub fn calculer_moyenne_temp(temperatures: &[f32]) -> Option<f32> {
        if temperatures.is_empty() {
            return None;
        }
        let sum: f32 = temperatures.iter().sum();
        let count = temperatures.len();
        Some(sum / count as f32)
    }

    #[derive(Debug)]
    pub enum Categorie {
        Froid,
        Tempere,
        Chaud,
    }

    pub fn categoriser_moyenne(moyenne: f32) -> Categorie {
        if moyenne < 10.0 {
            Categorie::Froid
        } else if moyenne <= 20.0 {
            Categorie::Tempere
        } else {
            Categorie::Chaud
        }
    }

    pub fn arrondir_a_deux_chiffres(valeur: f32) -> f32 {
        (valeur * 100.0).round() / 100.0
    }

    #[derive(Debug)]
    pub struct TemperatureMoyenne {
        pub moyenne: f32,
        pub categorie: Categorie,
    }

    pub fn obtenir_temperature_moyenne(temperatures: &[f32]) -> Option<TemperatureMoyenne> {
        match calculer_moyenne_temp(temperatures) {
            Some(moyenne) => {
                let moyenne_arrondie = arrondir_a_deux_chiffres(moyenne);
                let categorie = categoriser_moyenne(moyenne_arrondie);
                Some(TemperatureMoyenne { moyenne: moyenne_arrondie, categorie })
            }
            None => None,
        }
    }

    pub fn trier_temperatures(temperatures: &mut [f32]) {
        temperatures.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }
    
}