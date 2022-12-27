mod handler;
mod router;
mod server;

use server::Server;

fn main() {
  let server = Server::new("localhost:3000");
  println!("Server is running on http://localhost:3000");
  server.run();
}
