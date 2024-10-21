use zmq::{Context, Socket};

pub struct Zmq {
	context: Context,
}

impl Zmq {
	pub fn new() -> Self {
		Zmq {
			context: zmq::Context::new(),
		}
	}

	// Function to create a publisher
	pub fn create_publisher(&self, address: &str) -> Publisher {
		let socket = self
			.context
			.socket(zmq::PUB)
			.expect("Failed to create PUB socket");
		socket.bind(address).expect("Failed to bind PUB socket");
		Publisher { socket }
	}

	// Function to create a subscriber
	pub fn create_subscriber(&self, address: &str, filter: &str) -> Subscriber {
		let socket = self
			.context
			.socket(zmq::SUB)
			.expect("Failed to create SUB socket");
		socket
			.connect(address)
			.expect("Failed to connect SUB socket");
		socket
			.set_subscribe(filter.as_bytes())
			.expect("Failed to subscribe to filter");
		Subscriber { socket }
	}
}

pub struct Publisher {
	socket: Socket,
}

impl Publisher {
	pub fn send_message(&self, message: &str) {
		self.socket
			.send(message, 0)
			.expect("Failed to send message");
	}
}

pub struct Subscriber {
	socket: Socket,
}

impl Subscriber {
	pub fn receive_message(&self) -> String {
		self.socket
			.recv_string(0)
			.expect("Failed to receive message")
			.unwrap()
	}
}
