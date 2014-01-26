//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 0.9
//
// Note that this code has serious security risks!  You should not run it 
// on any system with access to sensitive files.
// 
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

#[feature(globs)];
use std::io::*;
use std::io::net::ip::{SocketAddr};
use std::os; 
use std::{str};

static IP: &'static str = "127.0.0.1";
static PORT:        int = 4414;
static mut count: int = 0; 

fn main() {
    let addr = from_str::<SocketAddr>(format!("{:s}:{:d}", IP, PORT)).unwrap();
    let mut acceptor = net::tcp::TcpListener::bind(addr).listen();
    
    println(format!("Listening on [{:s}] ...", addr.to_str()));
    
    for stream in acceptor.incoming() {
        // Spawn a task to handle the connection
        do spawn {
            let mut stream = stream;
          
            match stream {
                Some(ref mut s) => {
                             match s.peer_name() {
                                Some(pn) => {println(format!("Received connection from: [{:s}]", pn.to_str()));},
                                None => ()
                             }
                           },
                None => ()
	
            }
	
            
            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf);
            println(format!("Received request :\n{:s}", request_str));
            
            let response: ~str = 
                ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                 <doctype !html><html><head><title>Hello, Rust!</title>
		
                 <style>body { background-color: #111; color: #FFEEAA }
                        h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                        h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
                 </style></head>\
                 <body>
                 <h1>Greetings, Krusty!</h1>

                 </body></html>\r\n";
unsafe{
count = count+1; //counting page views
let response1: ~str = count.to_str(); //turning count into a string
let finalresponse: ~str = response.append(response1); //append count to hello message
stream.write(finalresponse.as_bytes()); //write that to screen

let cwd=os::getcwd(); //get directory of server
stream.write(cwd.as_str().unwrap().as_bytes()); //write to screen

let wordvec: ~[&str] = request_str.split(' ').collect(); //get requested file 
let path = Path::new(wordvec[1].clone()); //get path 
let msg_file=File::open(&path); 

match (msg_file) {
            Some(mut msg) => {
                let msg_bytes: ~[u8] = msg.read_to_end();
		let filestring= str::from_utf8(msg_bytes);  
				

		if wordvec[1].clone().contains(".html") && wordvec[1].clone().contains(cwd.as_str().unwrap()){ //if path is .html and in the same directory as server		
			stream.write("<body><h1> </h1>
		          	 </body></html>\r\n".as_bytes());  
			stream.write(filestring.as_bytes()); //write to screen 
		}
		else {stream.write("403 FORBIDDEN".as_bytes()); }//else write error 
               }, 

            None => fail!("Error opening message file:")
        
    }
	


}
	
           // stream.write(response.as_bytes());
	 
            println!("Connection terminates.");
        }
    }
}
