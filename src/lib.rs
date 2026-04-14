#![no_std]

// Copyright (C) 2026 Jorge Andre Castro - GPL-2.0-or-later

//! # embassy-hcsr505
//!
//! Driver async `no_std` pour le capteur de mouvement HC-SR505.
//!
//! ## Exemple d'utilisation
//!
//! ```rust,ignore
//! use embassy_hcsr505::Hcsr505;
//! 
//! #[embassy_executor::task]
//! async fn pir_task(pin: Input<'static>) {
//!     let mut pir = Hcsr505::new(pin);
//!     loop {
//!         pir.wait_for_motion().await;
//!         // Le signal global est mis à jour automatiquement !
//!         pir.wait_for_idle().await;
//!     }
//! }
//! ```

pub mod driver;
pub mod signals;

pub use driver::Hcsr505;