use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpusSourceError {
    #[error("Audio stream is not Opus format")]
    InvalidAudioStream,
    #[error("Invalid container format")]
    InvalidContainerFormat,
    #[error("Invalid header data")]
    InvalidHeaderData,
    #[cfg(feature = "with_ogg")]
    #[error("{0}")]
    OggHeaderError(#[from] ogg::OggReadError),
    #[error("Seeking not supported")]
    SeekingNotSupported,
    #[error("End of data stream")]
    EndOfDataStream,
}

// OggReadError
