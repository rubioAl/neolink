//! Captura del XML crudo de las respuestas de la cámara (rama «vigia»).
//!
//! Neolink convierte cada cuerpo XML en structs tipados ([`super::xml::BcXml`]) y descarta lo que no
//! conoce; para leer la configuración de la cámara mensaje a mensaje hace falta el texto tal cual.
//! Cuando la captura está activada (sólo la activa la orden `neolink xml`), el decodificador guarda
//! aquí el XML de cada respuesta, indexado por `msg_num`, y además no falla si el XML no encaja en
//! los structs conocidos (devuelve un `BcXml` vacío para que la respuesta llegue al que la pidió).

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

static ENABLED: AtomicBool = AtomicBool::new(false);
static STORE: Mutex<Vec<(u16, u32, String)>> = Mutex::new(Vec::new());
const MAX_KEPT: usize = 256;

/// Activa la captura del XML crudo para el resto del proceso.
pub fn enable_raw_capture() {
    ENABLED.store(true, Ordering::SeqCst);
}

/// ¿Está activada la captura?
pub fn enabled() -> bool {
    ENABLED.load(Ordering::SeqCst)
}

/// Guarda el XML de una respuesta (lo llama el decodificador). Si ya hay texto para ese `msg_num`
/// (respuesta partida en varios paquetes) lo concatena.
pub(crate) fn capture(msg_num: u16, msg_id: u32, text: &[u8]) {
    if !enabled() {
        return;
    }
    let text = String::from_utf8_lossy(text).into_owned();
    let mut store = STORE.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(entry) = store
        .iter_mut()
        .rev()
        .find(|(n, id, _)| *n == msg_num && *id == msg_id)
    {
        entry.2.push_str(&text);
        return;
    }
    store.push((msg_num, msg_id, text));
    if store.len() > MAX_KEPT {
        let drop = store.len() - MAX_KEPT;
        store.drain(0..drop);
    }
}

/// Saca (y borra) el XML guardado para una petición. Devuelve (msg_id, xml).
pub fn take_raw_xml(msg_num: u16) -> Option<(u32, String)> {
    let mut store = STORE.lock().unwrap_or_else(|e| e.into_inner());
    let pos = store.iter().rposition(|(n, _, _)| *n == msg_num)?;
    let (_, id, text) = store.remove(pos);
    Some((id, text))
}
