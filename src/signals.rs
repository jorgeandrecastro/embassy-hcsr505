
// Copyright (C) 2026 Jorge Andre Castro - GPL-2.0-or-later
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;

/// Signal global pour la détection de mouvement (true = mouvement, false = repos)
pub static MOTION_SIGNAL: Signal<CriticalSectionRawMutex, bool> = Signal::new();