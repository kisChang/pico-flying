use std::net::{TcpStream};
use std::io::{self, Write, Read};
use std::sync::Mutex;
use tauri::State;

struct TcpClient {
    stream: Option<TcpStream>,
}

impl TcpClient {
    fn new() -> Self {
        TcpClient { stream: None }
    }

    fn connect(&mut self, addr: &str) -> io::Result<()> {
        let stream: TcpStream = TcpStream::connect(addr)?;
        self.stream = Some(stream);
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.stream.is_some()
    }

    fn send_data(&mut self, data: &str) -> io::Result<usize> {
        if let Some(ref mut stream) = self.stream {
            stream.write(data.as_bytes())
        } else {
            Err(io::Error::new(io::ErrorKind::NotConnected, "No active TCP connection"))
        }
    }

    fn disconnect(&mut self) {
        self.stream = None; // 将流置为 None 即断开连接
    }
}

#[tauri::command]
fn connect_tcp(client: State<Mutex<TcpClient>>, addr: String) -> Result<String, String> {
    let mut client = client.lock().unwrap();
    match client.connect(&addr) {
        Ok(_) => Ok("Connected".into()),
        Err(e) => Err(format!("Failed to connect: {}", e)),
    }
}

#[tauri::command]
fn get_connection_status(client: State<Mutex<TcpClient>>) -> bool {
    let client = client.lock().unwrap();
    client.is_connected()
}

#[tauri::command]
fn send_data(client: State<Mutex<TcpClient>>, data: String) -> Result<usize, String> {
    let mut client = client.lock().unwrap();
    match client.send_data(&data) {
        Ok(size) => Ok(size),
        Err(e) => Err(format!("Failed to send data: {}", e)),
    }
}

#[tauri::command]
fn disconnect_tcp(client: State<Mutex<TcpClient>>) -> Result<String, String> {
    let mut client = client.lock().unwrap();
    client.disconnect();
    Ok("Disconnected".into())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(Mutex::new(TcpClient::new()))
        .invoke_handler(tauri::generate_handler![connect_tcp, get_connection_status, send_data, disconnect_tcp])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
