use twitch_irc::message::ServerMessage;

#[derive(Debug, Clone)]
pub enum Event {
    TwitchIRC(ServerMessage),
    DrumCommand(String),
    Stop,
    Play(String),
    QuirkMessage(String),
    OnCommand,
    OffCommand,
    StartOfProgram,
}


