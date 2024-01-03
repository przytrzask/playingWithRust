#[derive(Debug)]
struct URL {
    protocol: String,
    hostname: String,
    pathname: String,
}

impl URL {
    fn toString(&self) -> String {
        format!("{}://{}{}", self.protocol, self.hostname, self.pathname)
    }

    fn from(url: &str) -> URL {
        let string = String::from(url);
        let vec: Vec<&str> = string.split("://").collect();
        let protocol = String::from(vec[0]);
        let rest = String::from(vec[1]);
        let vec2: Vec<&str> = rest.split("/").collect();
        let hostname = String::from(vec2[0]); 
        let pathname = String::from(vec2[1]);
        return URL {
            protocol,
            hostname,
            pathname,
        };
    }
}

fn main() {
    let url = URL {
        protocol: String::from("https"),
        hostname: String::from("www.google.com"),
        pathname: String::from("/search?q=hello"),
    };

    let url2 = URL::from("https://www.google.com/search?q=hello");

    println!("url is {:?}", url.toString());
    println!("url2 is {:?}", url2);
}
