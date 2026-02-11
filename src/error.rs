pub enum GameError {
    EventLoopError(winit::error::EventLoopError),
    LevelTmxCloseElementFail,
    LevelTmxImageOutsideTileset,
    LevelTxmImageNoSourceProvided,
    LevelTmxPropertyUnhandledAttribute,
    #[allow(unused)]
    UnknownError,
}

impl std::fmt::Debug for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            GameError::EventLoopError(err) => {
                writeln!(f, "{err:?}")?;
            }
            other => {
                writeln!(f, "{other:?}")?;
            }
        }
        Ok(())
    }
}
