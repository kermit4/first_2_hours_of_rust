use std::env;
use std::fs::File;
use std::net::UdpSocket;
struct Upload {
	f: File,
	blocks: i32,
	firstmissing: i32,
	}
struct Content{
	len: i32,
	offset: i32,
	name: [char;256],  // should strip /s as this is absurdly insecure as-is
	data: [char;1024],
}
struct ReqAnother {
	offset: i32,
	name: [char;256],
}

fn main() {
    // The statements here will be executed when the compiled binary is called 
	let mut socket = UdpSocket::bind("0.0.0.0:34254");
	let args: Vec<String> = env::args().collect();
	if args.len() > 0 { // send a file
		println!("{}", args[1]);
	} else { //  receive files
		loop {
			use std::collections::HashMap;
			let mut uploads = HashMap::new();
			let mut c = Content{ };
		//	[0; ::std::mem::size_of::Content];
			let (amt, src) = socket.recv_from(&mut c)?;
			if ! uploads.contains_key(c.name) { // new upload
				let mut u = Upload {
				f: File::create(c.name),
				blocks: (c.len+1023) / 1024,
				firstmissing: 0,
				};
				uploads.insert(c.name,u)
			} else {
				let u = uploads.entry(c.name);
				let f = u.f;
				f.write_at(c.data, c.offset);
				let b = c.offset;
				if b == u.firstmissing {
					while u.bitmap.get(u.firstmissing) {
						u.firstmissing+=1;
					}
				}
				u.bitmap.set(c.offset,true);

				if u.blocks==0 { // uploade done
					uploads.remove(c.name);
					continue;
				}

				let mut m=ReqAnother {};
				if u.lastreq>u.blocks  {
					m.offset=u.firstmissing;
				} else {
					m.offset=u.lastreq;
				}
				u.lastreq+=1;
				m.name=u.name;

				socket.send_to(m,&src);
				
				if (m.offset%100) == 0 { // should be random, but I had a hard time importing that
					m.offset=u.firstmissing;
					socket.send_to(m,&src);
				}
			}
		}
	}
}


// 90 minutes of work


//// the following code i just copied from  https://stackoverflow.com/questions/38334994/how-to-read-a-c-struct-from-a-binary-file-in-rust
//use std::io::{self, BufReader, Read};
//use std::path::Path;
//use std::slice;
//
//fn read_struct<T, R: Read>(mut read: R) -> io::Result<T> {
//    let num_bytes = ::std::mem::size_of::<T>();
//    unsafe {
//        let mut s = ::std::mem::uninitialized();
//        let mut buffer = slice::from_raw_parts_mut(&mut s as *mut T as *mut u8, num_bytes);
//        match read.read_exact(buffer) {
//            Ok(()) => Ok(s),
//            Err(e) => {
//                ::std::mem::forget(s);
//                Err(e)
//            }
//        }
//    }
//}
