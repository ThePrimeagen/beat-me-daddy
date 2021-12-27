use twitch_irc::message::ServerMessage;

#[derive(Debug)]
pub enum Event {
    TwitchIRC(ServerMessage),
}


