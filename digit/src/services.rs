pub mod media;

use media::MediaService;
use parking_lot::{Mutex, MutexGuard};
use std::sync::Arc;

static mut SERVICES: Option<Services> = None;

pub struct Services {
    media_service: Arc<Mutex<MediaService>>,
}

impl Services {
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

    pub fn media() -> MutexGuard<'static, MediaService> {
        Self::instance().media_service.lock()
    }
}
