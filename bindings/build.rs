fn main() {
	windows::build! {
		Windows::{
			Foundation::{
				IReference,
				IAsyncOperation,
				TypedEventHandler
			},
			Media::{
				MediaPlaybackType,
				Control::*,
			},
		}
	};
}
