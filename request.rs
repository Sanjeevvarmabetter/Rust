fn main() {
    let get = Meth::GET;
    let delete = Meth::DELETE;
    let post = Meth::POST;
    let put = Meth::PUT;
    let server = Server::from("127.0.0.1:8080");
    server.run();


struct Server {
    addr: String,
}


impl Server {
    fn new(addr:String) -> Self {
         Self {
            addr: addr
         }
    }


    fn run(self) {
         println!("Listening on {}",self.addr);
    }
}

struct Request {
    path: String,
    query_string : String,
    method: String,
}
enum Meth {
    GET,
    DELETE,
    POST = 5,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/