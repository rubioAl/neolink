use clap::Parser;

/// Pide a la cámara mensajes del protocolo por su número y muestra el XML de cada respuesta tal cual
/// (rama «vigia»: lectura de la configuración de las Reolink Go PT Ultra a través del relay).
#[derive(Parser, Debug)]
pub struct Opt {
    /// The name of the camera. Must be a name in the config
    pub camera: String,
    /// Mensajes a pedir: `ID` (GET con cuerpo vacío) o `ID=<xml>` (con ese cuerpo), p. ej.
    /// 80 332 '342=<body><AiDetectCfg version="1.1"><chn>0</chn><type>people</type></AiDetectCfg></body>'
    #[arg(required = true)]
    pub ids: Vec<String>,
    /// Cuerpo XML a enviar con cada mensaje (por defecto, ninguno)
    #[arg(long)]
    pub body: Option<String>,
    /// Segundos de espera por cada respuesta
    #[arg(long, default_value_t = 8)]
    pub wait: u64,
}
