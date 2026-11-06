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
/// neolink xml --config=config.toml CameraName 80 '342=<body><AiDetectCfg version="1.1"><chn>0</chn><type>people</type></AiDetectCfg></body>'
/// neolink xml --config=config.toml CameraName 342 --body '<body><AiDetectCfg version="1.1"><chn>0</chn><type>vehicle</type></AiDetectCfg></body>'
/// ```
///
/// Salida: por cada número, una línea `### msg_id=N response_code=C` y después el XML (o
/// `(sin cuerpo XML)`). Si la cámara no contesta en `--wait` segundos, `response_code=0`.
///
use anyhow::{Context, Result};

mod cmdline;

use crate::common::NeoReactor;

pub(crate) use cmdline::Opt;

/// Antepone la declaración XML si el cuerpo no la trae (la cámara la espera)
fn with_declaration(xml: &str) -> String {
    let xml = xml.trim();
    if xml.starts_with("<?xml") {
        xml.to_string()
    } else {
        format!("<?xml version=\"1.0\" encoding=\"UTF-8\" ?>{}", xml)
    }
}

/// Entry point for the xml subcommand
pub(crate) async fn main(opt: Opt, reactor: NeoReactor) -> Result<()> {
    let camera = reactor.get(&opt.camera).await?;
    let wait = std::time::Duration::from_secs(opt.wait);

    for item in opt.ids.iter() {
        let (id, body) = match item.split_once('=') {
            Some((id, xml)) => (id.trim().parse::<u32>(), Some(with_declaration(xml))),
            None => (item.trim().parse::<u32>(), opt.body.clone().map(|b| with_declaration(&b))),
        };
        let id = match id {
            Ok(id) => id,
            Err(_) => {
                println!("### msg_id={} error=\"no es un número de mensaje\"", item);
                continue;
            }
        };
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
