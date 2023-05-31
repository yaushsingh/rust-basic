pub struct Server {  /* struct only holds data and actual functionality is written in implentation */
    addr: String,
}

impl Server { /*it is used to give functionality to struct Server */
    pub fn new(addr: String)-> Self{
        Self {
            addr  
        }
    }

    pub fn run(self){ /*we use pub here in this struct  and impl so that line 7 can access to modules name server */
        println!("lISTENING on {}", self.addr)
    }
}