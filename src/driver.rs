// Copyright (C) 2026 Jorge Andre Castro - GPL-2.0-or-later

use embassy_rp::gpio::Input;
use crate::signals::MOTION_SIGNAL;

pub struct Hcsr505<'d> {
    pin: Input<'d>,
}

impl<'d> Hcsr505<'d> {
    pub fn new(pin: Input<'d>) -> Self {
        Self { pin }
    }

    /// Attend une détection et publie sur le signal global
    pub async fn wait_for_motion(&mut self) {
        self.pin.wait_for_high().await;
        MOTION_SIGNAL.signal(true);
    }

    /// Attend la fin du mouvement et publie sur le signal global
    pub async fn wait_for_idle(&mut self) {
        self.pin.wait_for_low().await;
        MOTION_SIGNAL.signal(false);
    }

    /// Vérification instantanée
    pub fn is_detected(&self) -> bool {
        self.pin.is_high()
    }
}