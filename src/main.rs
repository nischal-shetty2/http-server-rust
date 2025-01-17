use std::{
	fs, io::{BufRead, BufReader, Write}, net::{TcpListener , TcpStream}, thread, time::Duration, 
};
use server::ThreadPool;

fn main() {
	let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();
	let pool = ThreadPool::new(4);
	for stream in listener.incoming() {
			let stream: TcpStream = stream.unwrap();
			pool.execute(||{
					handle_connection(stream);
				}
			);
	}
	println!("Shutting down.");
}
fn handle_connection(mut stream:TcpStream) ->(){
	let buff_reader= BufReader::new( &mut stream);
	let request_line: String = buff_reader.lines().next().unwrap().unwrap();

	let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5)); // to simulate a slow server response
            ("HTTP/1.1 200 OK", "sleep.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

	let contents = fs::read_to_string(filename).unwrap();
	let length = contents.len();
	let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
	stream.write_all(response.as_bytes()).unwrap()

}
