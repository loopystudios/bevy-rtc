mod receive;
mod send;

use crate::{
    protocol::Payload, schedule::SilkSchedule, sets::SilkSet,
    socket::common_socket_reader,
};
use bevy::prelude::*;
pub use receive::IncomingMessages;
pub use send::OutgoingMessages;

pub trait AddNetworkMessageExt {
    fn add_network_message<M: Payload>(&mut self) -> &mut Self;
}

impl AddNetworkMessageExt for App {
    fn add_network_message<M>(&mut self) -> &mut Self
    where
        M: Payload,
    {
        if self.world.contains_resource::<IncomingMessages<M>>()
            || self.world.contains_resource::<OutgoingMessages<M>>()
        {
            panic!("server already contains resource: {}", M::reflect_name());
        }
        self.insert_resource(IncomingMessages::<M> { messages: vec![] })
            .add_systems(
                SilkSchedule,
                IncomingMessages::<M>::flush.in_set(SilkSet::Flush),
            )
            .add_systems(
                SilkSchedule,
                IncomingMessages::<M>::read_system
                    .before(SilkSet::NetworkRead)
                    .after(common_socket_reader),
            )
            .insert_resource(OutgoingMessages::<M> {
                reliable_to_all: vec![],
                unreliable_to_all: vec![],
                reliable_to_all_except: vec![],
                unreliable_to_all_except: vec![],
                reliable_to_peer: vec![],
                unreliable_to_peer: vec![],
            })
            .add_systems(
                SilkSchedule,
                OutgoingMessages::<M>::write_system
                    .after(SilkSet::NetworkWrite),
            );

        self
    }
}