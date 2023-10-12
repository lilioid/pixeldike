use crate::pixmap::traits::{PixmapRead, PixmapWrite};
use crate::pixmap::SharedPixmap;
use crate::state_encoding::SharedMultiEncodings;
use crate::DaemonHandle;
use async_trait::async_trait;

/// A trait to unify the different transport protocol servers
#[async_trait]
pub trait GenServer {
    /// An options type with which the server can be configured
    type Options;

    /// Create a new server with the given options
    fn new(options: Self::Options) -> Self;

    /// Start the server in the background and return a handle with which the background
    /// task can be controlled.
    async fn start<P>(
        self,
        pixmap: SharedPixmap<P>,
        encodings: SharedMultiEncodings,
    ) -> anyhow::Result<DaemonHandle>
    where
        P: PixmapRead + PixmapWrite + Send + Sync + 'static;
}
