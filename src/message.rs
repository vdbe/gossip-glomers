use actix::prelude::*;
use anyhow::Context as AnyhowContext;
use serde::{Deserialize, Serialize};

use broadcast::{Broadcast, BroadcastOk};
use echo::{Echo, EchoOk};
use generate::{Generate, GenerateOk};
use gossip::{Gossip, GossipOk};
use init::{Init, InitOk};
use read::{Read, ReadOk};
use topology::{Topology, TopologyOk};

mod broadcast;
mod echo;
mod generate;
pub mod gossip;
mod init;
mod read;
mod topology;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorMessage<Payload> {
    pub message: GlommerMessage<()>,
    pub payload: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlommerMessage<Payload> {
    /// A string identifying the node this message came from
    pub src: String,

    /// A string identifying the node this message is to
    pub dest: String,

    // An object: the payload of the message
    pub body: GlommerBody<Payload>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlommerBody<Payload> {
    /// A unique integer identifier
    #[serde(rename = "msg_id")]
    pub id: Option<usize>,

    /// For req/response, the msg_id of the request
    pub in_reply_to: Option<usize>,

    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum GlommerPayload {
    Init(Init),
    InitOk(InitOk),
    Echo(Echo),
    EchoOk(EchoOk),
    Generate(Generate),
    GenerateOk(GenerateOk),
    Broadcast(Broadcast),
    BroadcastOk(BroadcastOk),
    Read(Read),
    ReadOk(ReadOk),
    Topology(Topology),
    TopologyOk(TopologyOk),
    Gossip(Gossip),
    GossipOk(GossipOk),
}

use super::MyActor;
impl MyActor {
    fn send(
        &mut self,
        dest: String,
        in_reply_to: Option<usize>,
        payload: GlommerPayload,
    ) -> anyhow::Result<usize> {
        let reply = GlommerMessage {
            src: self.node_id.clone(),
            dest,
            body: GlommerBody {
                id: Some(self.id),
                in_reply_to,
                payload,
            },
        };

        let mid = self.id;
        self.id += 1;

        reply
            .serialize(&mut self.output)
            .context("serialize response to init")?;

        Ok(mid)
    }

    fn reply(
        &mut self,
        message: GlommerMessage<()>,
        payload: GlommerPayload,
    ) -> anyhow::Result<()> {
        let reply = GlommerMessage {
            src: message.dest,
            dest: message.src,
            body: GlommerBody {
                id: Some(self.id),
                in_reply_to: message.body.id,
                payload,
            },
        };

        self.id += 1;

        reply
            .serialize(&mut self.output)
            .context("serialize response to init")
    }

    pub async fn send_glommer_message(
        addr: &Addr<MyActor>,
        glommer_message: GlommerMessage<GlommerPayload>,
    ) -> anyhow::Result<()> {
        let message: GlommerMessage<()> = GlommerMessage {
            src: glommer_message.src,
            dest: glommer_message.dest,
            body: GlommerBody {
                id: glommer_message.body.id,
                in_reply_to: glommer_message.body.in_reply_to,
                payload: (),
            },
        };

        // TODO: Find a better way for this
        let response = match glommer_message.body.payload {
            GlommerPayload::Init(init) => {
                let actor_message = ActorMessage {
                    message,
                    payload: init,
                };

                addr.send(actor_message).await
            }
            GlommerPayload::InitOk(init_ok) => {
                let actor_message = ActorMessage {
                    message,
                    payload: init_ok,
                };

                addr.send(actor_message).await
            }
            GlommerPayload::Echo(echo) => {
                let actor_message = ActorMessage {
                    message,
                    payload: echo,
                };

                addr.send(actor_message).await
            }
            GlommerPayload::EchoOk(echo_ok) => {
                let actor_message = ActorMessage {
                    message,
                    payload: echo_ok,
                };

                addr.send(actor_message).await
            }
            GlommerPayload::Generate(generate) => {
                let actor_message = ActorMessage {
                    message,
                    payload: generate,
                };

                addr.send(actor_message).await
            }
            GlommerPayload::GenerateOk(generate_ok) => {
                let actor_message = ActorMessage {
                    message,
                    payload: generate_ok,
                };

                addr.send(actor_message).await
            }
            GlommerPayload::Broadcast(broadcast) => {
                let actor_message = ActorMessage {
                    message,
                    payload: broadcast,
                };

                addr.send(actor_message).await
            }
            GlommerPayload::BroadcastOk(broadcast_ok) => {
                let actor_message = ActorMessage {
                    message,
                    payload: broadcast_ok,
                };

                addr.send(actor_message).await
            }
            GlommerPayload::Read(read) => {
                let actor_message = ActorMessage {
                    message,
                    payload: read,
                };

                addr.send(actor_message).await
            }
            GlommerPayload::ReadOk(read_ok) => {
                let actor_message = ActorMessage {
                    message,
                    payload: read_ok,
                };

                addr.send(actor_message).await
            }
            GlommerPayload::Topology(topology) => {
                let actor_message = ActorMessage {
                    message,
                    payload: topology,
                };

                addr.send(actor_message).await
            }
            GlommerPayload::TopologyOk(topology_ok) => {
                let actor_message = ActorMessage {
                    message,
                    payload: topology_ok,
                };

                addr.send(actor_message).await
            }
            GlommerPayload::Gossip(gossip) => {
                let actor_message = ActorMessage {
                    message,
                    payload: gossip,
                };
                addr.send(actor_message).await
            }
            GlommerPayload::GossipOk(gossip_ok) => {
                let actor_message = ActorMessage {
                    message,
                    payload: gossip_ok,
                };

                addr.send(actor_message).await
            }
        };

        response
            .context("send to handler")?
            .context("handler failed")
    }
}
