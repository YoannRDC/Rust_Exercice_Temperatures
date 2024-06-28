// *********
// Objectifs Pédagogiques
// *********

Apprendre à créer et utiliser des fonctions et des modules en Rust.
Comprendre l'utilisation des énumérations (enum) et des structures (struct).
Apprendre à manipuler les entrées/sorties en Rust.
Pratiquer la gestion des erreurs et le traitement des données utilisateur.

// *********
// Exercice Complet - Application Rust
// *********

Cet exercice consiste à coder une application Rust qui permet de calculer et d'afficher la moyenne des températures, de les catégoriser et de gérer diverses fonctionnalités associées. Vous allez créer des fonctions, des structures, et utiliser des énumérations pour structurer votre code.

// *********
// Consignes de l'Exercice
// *********

Exo 1: Créer la fonction calculer_moyenne_temp qui renvoie la moyenne des températures.

Exo 2: Afficher la moyenne avec deux chiffres après la virgule.

Exo 3: Catégoriser la moyenne :
Froid si la moyenne < 10
Tempéré si la moyenne est entre 10 et 20
Chaud si la moyenne > 20
Afficher un message selon la catégorie en utilisant une énumération (enum).

Exo 4: Créer une structure TemperatureMoyenne contenant la moyenne et la catégorie (struct).

Exo 5: Afficher un message "Aucune température" si le tableau des températures est vide (return).

Exo 6: Optionnel : Ajouter une fonctionnalité pour trier les températures et les afficher dans l'ordre croissant.

Exo 7: Optionnel : Lire les températures depuis l'entrée de l'utilisateur.
Si l'entrée n'est pas un nombre valide, renvoyer une erreur (std::io).

// *********
// Structure du Projet
// *********

Le projet se compose de deux fichiers principaux :

1. main.rs : Le fichier principal qui contient la fonction main et les appels aux fonctions définies dans le module temp.
2. temperature.rs : Un module contenant les fonctions et structures nécessaires pour l'exercice.

// *********
Instructions pour exécuter le code.
// *********

Clonez le dépôt ou créez un nouveau projet Rust.
Compilez et exécutez le projet en utilisant "cargo run".
Suivez les instructions pour entrer les températures via la console.

// *********
// Résultat de l'exécution
// *********

Exo 1: Moyenne: 21.285715
Exo 2: Moyenne arrondie: 21.29
Exo 3: Moyenne catégorisée: Chaud
Exo 4: Température moyenne via Objet: Some(TemperatureMoyenne { moyenne: 21.29, categorie: Chaud })
Exo 5: Aucune température
Exo 6: Température moyenne via Objet: [18.0, 19.5, 20.0, 21.0, 22.0, 23.5, 25.0]
Exo 7: Entrez une température (ou 'fin' pour terminer) : 8.2
Exo 7: Entrez une température (ou 'fin' pour terminer) : 9.4
Exo 7: Entrez une température (ou 'fin' pour terminer) : 14
Exo 7: Entrez une température (ou 'fin' pour terminer) : -0.2
Exo 7: Entrez une température (ou 'fin' pour terminer) : -12.3
Exo 7: Entrez une température (ou 'fin' pour terminer) : 25.1523
Exo 7: Entrez une température (ou 'fin' pour terminer) : fin
Exo 7: Températures triées: [-12.3, -0.2, 8.2, 9.4, 14.0, 25.1523]
Exo 7: Moyenne température: 7.38°C, Catégorie: Froid