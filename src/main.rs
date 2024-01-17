use std::fmt::format;

use buttplug::{
	client::{ButtplugClient, ButtplugClientEvent, LinearCommand, ScalarValueCommand},
	core::{
		connector::{ButtplugRemoteClientConnector, ButtplugWebsocketClientTransport},
		message::serializer::ButtplugClientJSONSerializer,
	},
};
use futures::{SinkExt, StreamExt};
use once_cell::sync::Lazy;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use serde::{Deserialize, Serialize};

type ButtplugConnector = ButtplugRemoteClientConnector<ButtplugWebsocketClientTransport, ButtplugClientJSONSerializer>;

static ADDR: Lazy<String> = Lazy::new(|| {
	let rng = rand::random::<[u8; 6]>();
	format!(
		"{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
		rng[0], rng[1], rng[2], rng[3], rng[4], rng[5]
	)
});

#[derive(Debug, Deserialize, Serialize)]
struct Handshake {
	identifier: String,
	address: String,
	version: u32,
}

impl Handshake  {
	fn new() -> Self {
		Self {
			identifier: String::from("HandyCoordinator"),
			// identifier: String::from("LVSDevice"),
			address: ADDR.clone(),
			version: 0,
		}
	}
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (mut ws_stream, _) = connect_async("ws://127.0.0.1:54817").await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed.");
	ws_stream.send(Message::Text(serde_json::to_string(&Handshake::new()).unwrap())).await.unwrap();

	while let Some(msg) = ws_stream.next().await {
		let Ok(msg) = msg else {
			eprintln!("Error receiving message: {msg:?}");
			continue;
		};

		match msg {
			Message::Text(msg) => {
				println!("Received text message: {msg:?}");
				match msg {
					msg if msg.starts_with("DeviceType") => {
						// [identifier]:[bluetooth device address with no colons]:[firmware version]
						ws_stream.send(Message::Text(format!("Z:{}:10", ADDR.clone()))).await.unwrap();
					},
					x => {
						println!("Unknown text message: {x}");
					}
				}
			},
			x => {
				println!("Received message: {x:?}");
			}
		}

	}

	// let connector = ButtplugConnector::new(
		// ButtplugWebsocketClientTransport::new_insecure_connector("ws://127.0.0.1:12345")
	// );

	// let websocket_connector = ButtplugConnector::new(
		// ButtplugWebsocketClientTransport::new_insecure_connector("ws://127.0.0.1:54817")
	// );
//
	// let client = ButtplugClient::new("Example Client");
	// client.connect(websocket_connector).await.expect("Can't connect to Buttplug Server, exiting!");
//
	// let mut event_stream = client.event_stream();
//
	// // As an example of event messages, we'll assume the server might
	// // send the client notifications about new devices that it has found.
	// // The client will let us know about this via events.
	// while let Some(event) = event_stream.next().await {
		// if let ButtplugClientEvent::DeviceAdded(device) = event {
			// println!("Device {:#?} connected", device);
//
			// // let v = device.vibrate(0.5).await;
			// // println!("Vibrate result: {:?}", v);
//
			// // let o = device.oscillate(&ScalarValueCommand::ScalarValue(50.)).await;
			// // println!("Oscillate result: {:?}", o);
//
			// let l = device.linear(&LinearCommand::Linear(1000, 1.)).await;
			// println!("Linear result: {:?}", l);
			// tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
//
			// let _ = device.linear(&LinearCommand::Linear(1000, 0.)).await;
		// }
	// }

	Ok(())
}

