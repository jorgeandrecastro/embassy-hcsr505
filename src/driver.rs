// Copyright (C) 2026 Jorge Andre Castro - GPL-2.0-or-later

use embedded_hal_async::digital::Wait;
use crate::signals::MOTION_SIGNAL;

pub struct Hcsr505<T: Wait> {
    pin: T,
}

impl<T: Wait> Hcsr505<T> {
    pub fn new(pin: T) -> Self {
        Self { pin }
    }

    /// Attend une détection et publie sur le signal global
    pub async fn wait_for_motion(&mut self) {
        // Le trait Wait fournit wait_for_high
        let _ = self.pin.wait_for_high().await;
        MOTION_SIGNAL.signal(true);
    }

    /// Attend la fin du mouvement et publie sur le signal global
    pub async fn wait_for_idle(&mut self) {
        // Le trait Wait fournit wait_for_low
        let _ = self.pin.wait_for_low().await;
        MOTION_SIGNAL.signal(false);
    }
}