///
/// # Neolink Xml (rama «vigia»)
///
/// Pide mensajes del protocolo Baichuan por su número y muestra el XML crudo de la respuesta.
/// Sirve para leer la configuración de la cámara (punto de guardia, detección, máscaras…)
/// sin tener que enseñar a Neolink la estructura de cada mensaje.
///
/// # Usage
///
/// ```bash
/// neolink xml --config=config.toml CameraName 80 199 332
/// neolink xml --config=config.toml CameraName 342 --body '<?xml version="1.0" encoding="UTF-8" ?><body><AiDetectCfg version="1.1"><chn>0</chn><type>people</type></AiDetectCfg></body>'
/// ```
///
/// Salida: por cada número, una línea `### msg_id=N response_code=C` y después el XML (o
/// `(sin cuerpo XML)`). Si la cámara no contesta en `--wait` segundos, `response_code=0`.
///
use anyhow::{Context, Result};

mod cmdline;

use crate::common::NeoReactor;

pub(crate) use cmdline::Opt;

/// Entry point for the xml subcommand
pub(crate) async fn main(opt: Opt, reactor: NeoReactor) -> Result<()> {
    let camera = reactor.get(&opt.camera).await?;
    let wait = std::time::Duration::from_secs(opt.wait);

    for id in opt.ids.iter().copied() {
        let body = opt.body.clone();
        let result = camera
            .run_task(move |cam| {
                let body = body.clone();
                Box::pin(async move {
                    cam.get_raw_xml(id, body, wait)
                        .await
                        .with_context(|| format!("msg_id {}", id))
                })
            })
            .await;
        match result {
            Ok(reply) => {
                println!("### msg_id={} response_code={}", reply.msg_id, reply.response_code);
                match reply.xml {
                    Some(text) => println!("{}", text.trim_end()),
                    None => println!("(sin cuerpo XML)"),
                }
            }
            Err(e) => {
                println!("### msg_id={} error={:?}", id, e);
            }
        }
    }

    Ok(())
}
