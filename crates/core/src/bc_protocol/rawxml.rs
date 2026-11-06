//! Petición GET genérica por número de mensaje, con la respuesta en XML crudo (rama «vigia»).

use super::{BcCamera, Result};
use crate::bc::{model::*, rawxml};
use tokio::time::{timeout, Duration};

/// Respuesta cruda de la cámara a un mensaje pedido por su número
#[derive(Debug, Clone)]
pub struct RawReply {
    /// Número de mensaje pedido
    pub msg_id: u32,
    /// Código de respuesta de la cabecera (200 OK, 400 rechazado…); 0 si no contestó a tiempo
    pub response_code: u16,
    /// XML del cuerpo de la respuesta, si lo hubo
    pub xml: Option<String>,
}

impl BcCamera {
    /// Envía el mensaje `msg_id` (cabecera + `Extension` con el canal y, si se da, `body` como XML
    /// escrito a mano) y devuelve el XML crudo de la respuesta, esperando como mucho `wait`.
    pub async fn get_raw_xml(
        &self,
        msg_id: u32,
        body: Option<String>,
        wait: Duration,
    ) -> Result<RawReply> {
        rawxml::enable_raw_capture();
        let connection = self.get_connection();
        let msg_num = self.new_message_num();
        let mut sub = connection.subscribe(msg_id, msg_num).await?;

        let msg = Bc {
            meta: BcMeta {
                msg_id,
                channel_id: self.channel_id,
                msg_num,
                stream_type: 0,
                response_code: 0,
                class: 0x6414,
            },
            body: BcBody::ModernMsg(ModernMsg {
                extension: Some(Extension {
                    channel_id: Some(self.channel_id),
                    ..Default::default()
                }),
                payload: body.map(BcPayloads::RawXml),
            }),
        };
        sub.send(msg).await?;

        match timeout(wait, sub.recv()).await {
            Ok(reply) => {
                let reply = reply?;
                Ok(RawReply {
                    msg_id,
                    response_code: reply.meta.response_code,
                    xml: rawxml::take_raw_xml(msg_num).map(|(_, text)| text),
                })
            }
            Err(_) => Ok(RawReply {
                msg_id,
                response_code: 0,
                xml: None,
            }),
        }
    }
}
