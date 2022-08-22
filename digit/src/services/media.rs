use bindings::Windows::Foundation::TypedEventHandler;
pub use bindings::Windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession as MediaSession,
    GlobalSystemMediaTransportControlsSessionManager as MediaSessionManager,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus as MediaPlaybackStatus,
};
use parking_lot::Mutex;
use std::sync::Arc;

pub struct MediaService {
    _session_manager: Option<MediaSessionManager>,
    callbacks: Vec<Box<dyn Fn(&Option<MediaSession>)>>,
}

impl MediaService {
    /**
     * Only `services.rs` should be able to construct services
     * 
     * Comes prewrapped in an ArcMutex, you're welcome :)
     */
    pub(super) fn new() -> Arc<Mutex<MediaService>> {
        let media_service = Arc::new(Mutex::new(MediaService {
            _session_manager: None,
            callbacks: Vec::new(),
        }));

        // Hang onto a weak reference 
        let weak_media_service = Arc::downgrade(&media_service);

        let session_manager = MediaSessionManager::RequestAsync().unwrap().get().unwrap();
        let current_session_result = session_manager.GetCurrentSession();
        if let Ok(session) = current_session_result {
            session
                .PlaybackInfoChanged(TypedEventHandler::new(move |session, _args| {
                    if let Some(ms) = weak_media_service.upgrade() {
                        ms.lock().notify_all(session);
                    }
                    Ok(())
                }))
                .unwrap();
            media_service.lock()._session_manager = Some(session_manager);
        }
        media_service
    }

    pub fn subscribe<F>(&mut self, callback: F)
    where
        F: 'static + Fn(&Option<MediaSession>),
    {
        self.callbacks.push(Box::new(callback));
    }

    pub fn notify_all(&self, session: &Option<MediaSession>) {
        for c in &self.callbacks {
            c(session);
        }
    }

    pub fn get_media_session(&self) -> Option<MediaSession> {
        MediaSessionManager::RequestAsync()
            .unwrap()
            .get()
            .unwrap()
            .GetCurrentSession()
            .ok()
    }
}
