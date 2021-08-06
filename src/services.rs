pub mod media;

use media::MediaService;
use std::sync::{Arc, Mutex, MutexGuard};

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
            match SERVICES {
                Some(ref services) => services,
                None => panic!("what even"),
            }
        }
    }

    pub fn media() -> MutexGuard<'static, MediaService> {
        Self::instance().media_service.lock().unwrap()
    }
}
