use crate::network::packets::packet::Packet;

pub struct Client{
    gameprofile: GameProfile
}

pub struct GameProfile {
    uuid: String,
    username: String,
    properties: GameProfileProperties
}

pub struct GameProfileProperties {
    name: String,
    value: String,
    signature: Option<String>
}

impl Client{
    fn send_packet(packet: impl Packet){
        
    }
}