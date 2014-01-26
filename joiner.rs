use std::rand::random;
use std::os;
use std::io::File;

fn main() {
    let args: ~[~str] = os::args();
    if args.len() != 3 {
        println!("Usage: {:s} <inputfile>", args[0]); 
    } else {
	let mut vec = ~args; 
        let fname = vec[1].clone();
	let fname2= vec[2].clone(); 
        let path = Path::new(fname.clone());
        let msg_file = File::open(&path);
	let path2 = Path::new(fname2.clone()); 
	let msg_file2 = File::open(&path2); 

        match (msg_file, msg_file2) {
           ( Some(mut msg), Some(mut msg2)) => {
                let msg_bytes: ~[u8] = msg.read_to_end();
		let msg_bytes2: ~[u8] = msg2.read_to_end(); 
		let file_join = File::create(&Path::new("finish.txt"));

	match (file_join) {
                    Some(filejoin) => { 
                        join(msg_bytes, msg_bytes2, filejoin); 
                        } ,
                    _ => fail!("Error opening output files!"),
                }
            } ,
            (_,_) => fail!("Error opening message file: {:s}", fname)
        }
    }
}

fn xor(a: &[u8], b: &[u8]) -> ~[u8] {
    let mut ret = ~[];
    for i in range(0, a.len()) {
	ret.push(a[i] ^ b[i]);
    }
    ret
}

fn join(msg_bytes: &[u8], msg_bytes2: &[u8], mut file_join: File) {
      
    let decrypted_bytes = xor(msg_bytes, msg_bytes2);
    file_join.write(decrypted_bytes);
}
