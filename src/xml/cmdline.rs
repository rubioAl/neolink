use clap::Parser;

/// Pide a la cámara mensajes del protocolo por su número y muestra el XML de cada respuesta tal cual
/// (rama «vigia»: lectura de la configuración de las Reolink Go PT Ultra a través del relay).
#[derive(Parser, Debug)]
pub struct Opt {
    /// The name of the camera. Must be a name in the config
    pub camera: String,
    /// Números de mensaje a pedir (GET con cuerpo vacío), p. ej. 80 199 332
    #[arg(required = true)]
    pub ids: Vec<u32>,
    /// Cuerpo XML a enviar con cada mensaje (por defecto, ninguno)
    #[arg(long)]
    pub body: Option<String>,
    /// Segundos de espera por cada respuesta
    #[arg(long, default_value_t = 8)]
    pub wait: u64,
}
