pub mod media;

use media::MediaService;
use parking_lot::{Mutex, MutexGuard};
use std::sync::Arc;

static mut SERVICES: Option<Services> = None;

/**
 * Static struct that contains all services
 * 
 * MUTABLE STATIC DATA IS DANGEROUS, BE VERY CAREFUL
 * CONTAIN ALL DATA IN THREAD-SAFE STRUCTURES
 */
pub struct Services {
    media_service: Arc<Mutex<MediaService>>,
}

impl Services {
    /**
     * Get an instance of the Services singleton
     * 
     * Uses lazy singleton construction pattern, call early if
     * this takes a while
     */
    fn instance() -> &'static Services {
        unsafe {
            if SERVICES.is_none() {
                SERVICES = Some(Services {
                    media_service: MediaService::new(),
                })
            }
            SERVICES.as_ref().unwrap()
        }
    }

    /**
     * Lock and get a reference to MediaService
     * 
     * Unlocks as it goes out of scope, or with a `drop()` if you
     * need it to be unlocked earlier
     */
    pub fn media() -> MutexGuard<'static, MediaService> {
        Self::instance().media_service.lock()
    }
}
