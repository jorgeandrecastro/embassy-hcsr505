# embassy-hcsr505
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
embassy-hcsr505 = { version = "0.1.1" }
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


# Exemple branchement GP20 et 5V  , Oled ( ma crate) et le blink de la pico 2.


```rust

#![no_std]
#![no_main]

use cortex_m_rt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::i2c::{Config as I2cConfig, I2c, Async};
use embassy_time::{Delay, Duration, Timer};
use hd44780_i2c_nostd::LcdI2c;
use {panic_halt as _, embassy_rp as _};
use heapless::String;
use core::fmt::Write;

// MA CRATE 🦅 
use embassy_hcsr505::Hcsr505;
use embassy_hcsr505::signals::MOTION_SIGNAL;

use rp2350_linker as _;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::{I2C0, PIN_20}; 
use embassy_rp::Peri; 

bind_interrupts!(struct Irqs {
    I2C0_IRQ => embassy_rp::i2c::InterruptHandler<I2C0>;
});

// TASK : DETECTION PIR VIA CRATE 
#[embassy_executor::task]
async fn pir_task(pin_p: Peri<'static, PIN_20>) {
    // On crée l'input classique
    let pin = Input::new(pin_p, Pull::Down);
    
    // On initialise ton driver Eagle
    let mut pir = Hcsr505::new(pin);

    loop {
        // gère l'attente ET la publication sur MOTION_SIGNAL
        pir.wait_for_motion().await;
        pir.wait_for_idle().await;
    }
}

// TASK : DISPLAY JC-OS SECURITY 
#[embassy_executor::task]
async fn display_task(mut lcd: LcdI2c<I2c<'static, I2C0, Async>>) {
    let mut delay = Delay;
    let mut count = 0u32; 

    Timer::after(Duration::from_millis(500)).await;
    
    if lcd.init(&mut delay).await.is_ok() {
        let _ = lcd.set_backlight(true);
        let _ = lcd.clear(&mut delay).await;
        let _ = lcd.write_str("   JC-OS KERNEL", &mut delay).await;
        let _ = lcd.set_cursor(1, 0, &mut delay).await;
        let _ = lcd.write_str("   SECURE MODE", &mut delay).await;
    }

    Timer::after(Duration::from_secs(2)).await;

    loop {
        // On attend le signal venant de la crate
        let detected = MOTION_SIGNAL.wait().await;
        
        let _ = lcd.clear(&mut delay).await;
        let _ = lcd.set_cursor(0, 0, &mut delay).await;

        if detected {
            count += 1;
            let mut s: String<16> = String::new();
            let _ = write!(s, "INTRUSION #{}", count);
            
            let _ = lcd.write_str(s.as_str(), &mut delay).await;
            let _ = lcd.set_cursor(1, 0, &mut delay).await;
            let _ = lcd.write_str(" EAGLE ALERT :)", &mut delay).await;
        } else {
            let _ = lcd.write_str("  SYSTEM READY", &mut delay).await;
            let _ = lcd.set_cursor(1, 0, &mut delay).await;
            let _ = lcd.write_str("   ALL CLEAR", &mut delay).await;
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(embassy_rp::config::Config::default());

    // On initialise le signal pour que l'écran ne soit pas bloqué au boot
    MOTION_SIGNAL.signal(false); 

    let mut i2c_config = I2cConfig::default();
    i2c_config.frequency = 100_000;
    let i2c = I2c::new_async(p.I2C0, p.PIN_5, p.PIN_4, Irqs, i2c_config);
    let lcd = LcdI2c::new(i2c, 0x3F); 

    // Lancement des tâches
    spawner.spawn(pir_task(p.PIN_20)).unwrap();
    spawner.spawn(display_task(lcd)).unwrap();

    let mut led = Output::new(p.PIN_25, Level::Low);
    loop {
        led.toggle();
        Timer::after_millis(500).await; 
    }
}
```


Licence
Copyright (C) 2026 Jorge Andre Castro

Ce programme est un logiciel libre ; vous pouvez le redistribuer et/ou le modifier selon les termes de la Licence Publique Générale GNU (GPL-2.0-or-later).