[![Docs](https://docs.rs/embassy-hcsr505/badge.svg)](https://docs.rs/embassy-hcsr505)
[![Crates.io](https://img.shields.io/crates/v/embassy-hcsr505.svg)](https://crates.io/crates/embassy-hcsr505)
[![License](https://img.shields.io/crates/l/embassy-hcsr505.svg)](LICENSE)

# embassy-hcsr505
Driver asynchrone no_std pour le capteur de mouvement PIR HC-SR505, optimisé pour l'écosystème Embassy.

**Caractéristiques**
100% Asynchrone : Utilise les interruptions matérielles pour ne pas bloquer le processeur.

**Signaux Intégrés:** Publication automatique de l'état sur un signal global (MOTION_SIGNAL).

Empreinte minimale : Conçu pour les microcontrôleurs à ressources limitées (RP2040/RP2350, STM32, etc.).

----

**Installation**
Ajoutez ceci à votre fichier Cargo.toml :

````
[dependencies]
embassy-hcsr505 = { version = "0.1.2" }
````
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
----

**Architecture**
Le capteur HC-SR505 fonctionne sur une logique de seuil simple. Lorsqu'un corps thermique traverse le champ de vision, la sortie passe à l'état haut (3.3V) pendant une durée prédéfinie par le matériel (environ 8 secondes).

------

## Exemple simple : Détection PIR avec HC-SR505

**Branchement du capteur HC-SR505 (Pico 2) :**
- Pin OUT (capteur) → GP20
- Pin 5V → 5V (Pico)
- Pin GND → GND (Pico)

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Pull};
use embassy_time::Timer;
use {panic_halt as _, embassy_rp as _};

use embassy_hcsr505::Hcsr505;
use embassy_hcsr505::signals::MOTION_SIGNAL;

// Tâche 1 : Détection du mouvement avec le driver
#[embassy_executor::task]
async fn motion_sensor_task(pin: embassy_rp::peripherals::PIN_20) {
    let input = Input::new(pin, Pull::Down);
    let mut pir = Hcsr505::new(input);

    loop {
        // Attend une détection de mouvement
        // Le driver publie automatiquement sur MOTION_SIGNAL
        pir.wait_for_motion().await;

        // Attend la fin du mouvement
        pir.wait_for_idle().await;
    }
}

// Tâche 2 : Écouter les événements du capteur
#[embassy_executor::task]
async fn alert_task() {
    loop {
        let motion_detected = MOTION_SIGNAL.wait().await;

        if motion_detected {
            defmt::println!("🚨 Mouvement détecté !");
        } else {
            defmt::println!("✓ Zone sécurisée");
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(embassy_rp::config::Config::default());

    // Initialiser le signal
    MOTION_SIGNAL.signal(false);

    // Lancer les tâches
    spawner.spawn(motion_sensor_task(p.PIN_20)).unwrap();
    spawner.spawn(alert_task()).unwrap();
}
```
-----

# Licence
Copyright (C) 2026 Jorge Andre Castro

Ce programme est un logiciel libre ; vous pouvez le redistribuer et/ou le modifier selon les termes de la Licence Publique Générale GNU (GPL-2.0-or-later).