// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

extern crate pubsub;
extern crate amqp;

use pubsub::{PubSub, Pub};
use std::time::{Duration, SystemTime};
use std::thread;
use std::env;
use std::process::exit;
use amqp::{Consumer, Channel, protocol, Basic};

pub struct MyHandler {
    push: Pub,
    count: u64,
    start: SystemTime,
    max: u64,
}

impl MyHandler {
    pub fn new(ps: Pub,max: u64) -> Self {
        MyHandler {
            push:ps,
            count: 0,
            start: SystemTime::now(),
            max: max,
        }
    }
}

impl Consumer for MyHandler {
    fn handle_delivery(&mut self,
                       channel: &mut Channel,
                       deliver: protocol::basic::Deliver,
                       _: protocol::basic::BasicProperties,
                       _: Vec<u8>) {
        self.count = self.count + 1;
        let _ = channel.basic_ack(deliver.delivery_tag, false);
        self.push.publish("latency_req", tx_size());
        if self.count == self.max {
            let sys_time = SystemTime::now();
            let diff = sys_time.duration_since(self.start)
                .expect("SystemTime::duration_since failed");
            println!{"count {:?}, timer diff: {:?}", self.count, diff};
            thread::sleep(Duration::new(2,0));
            exit(0);
        }
    }
}
fn tx_size() ->Vec<u8>{
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("need only two argument : max msg count!");
    }
    let size = args[2].parse::<usize>().unwrap(); 

    //let size:usize = 1211656;
    let filesize = vec![0; size];
    filesize
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("need only two argument : max msg count!");
        return;
    }
    let max = args[1].parse::<u64>().unwrap();
    let mut pubsub = PubSub::new();
    let mut _pub = pubsub.get_pub(); 
    _pub.publish("latency_req", tx_size());
    pubsub.start_sub("latency_req", vec!["latency_res"], MyHandler::new(_pub, max));
    
    loop {
        thread::sleep(Duration::new(10,0));
    }
}
