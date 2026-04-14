embassy-hcsr505
Driver asynchrone no_std pour le capteur de mouvement PIR HC-SR505, optimisé pour l'écosystème Embassy.

Caractéristiques
100% Asynchrone : Utilise les interruptions matérielles pour ne pas bloquer le processeur.

Signaux Intégrés : Publication automatique de l'état sur un signal global (MOTION_SIGNAL).

Empreinte minimale : Conçu pour les microcontrôleurs à ressources limitées (RP2040/RP2350, STM32, etc.).

Licence Libre : Distribué sous GPL-2.0-or-later.

Installation
Ajoutez ceci à votre fichier Cargo.toml :

Ini, TOML
[dependencies]
embassy-hcsr505 = { version = "0.1.0" }
Utilisation
Le driver a été conçu pour être simple : vous lui donnez une pin configurée en entrée, et il gère le reste.

1. Configuration du Driver
```rust
use embassy_hcsr505::Hcsr505;
use embassy_rp::gpio::{Input, Pull};

#[embassy_executor::task]
async fn motion_sensor_task(pin: Input<'static>) {
    let mut pir = Hcsr505::new(pin);

    loop {
        // Attend qu'un mouvement soit détecté (Front montant)
        // Met automatiquement MOTION_SIGNAL à 'true'
        pir.wait_for_motion().await;

        // Attend que le capteur revienne au repos (Front descendant)
        // Met automatiquement MOTION_SIGNAL à 'false'
        pir.wait_for_idle().await;
    }
}
```
2. Écouter les détections ailleurs dans le code
Grâce au signal global, n'importe quelle autre tâche (affichage, alarme) peut réagir au mouvement.

```rust
use embassy_hcsr505::signals::MOTION_SIGNAL;

#[embassy_executor::task]
async fn alert_task() {
    loop {
        let detected = MOTION_SIGNAL.wait().await;
        if detected {
            // Logique en cas d'intrusion
        }
    }
}
```
Architecture
Le capteur HC-SR505 fonctionne sur une logique de seuil simple. Lorsqu'un corps thermique traverse le champ de vision, la sortie passe à l'état haut (3.3V) pendant une durée prédéfinie par le matériel (environ 8 secondes).

Licence
Copyright (C) 2026 Jorge Andre Castro

Ce programme est un logiciel libre ; vous pouvez le redistribuer et/ou le modifier selon les termes de la Licence Publique Générale GNU (GPL-2.0-or-later).