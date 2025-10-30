/// A request from the client to the server
#[derive(Debug, PartialEq)]
pub enum Request {
    /// Add the document `doc` to the archive
    Publish { doc: String },
    /// Search for the word `word` in the archive
    Search { word: String },
    /// Retrieve the document with the index `id` from the archive
    Retrieve { id: usize },
}
impl Request {
    // Convert the request `self` into a byte vector. See the assignment handout for suggestions on
    // how to represent the request as a series of bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            Request::Publish { doc } => {
                bytes_add_usize(&mut bytes, 0);
                bytes_add_string(&mut bytes, doc);
            }
            Request::Search { word } => {
                bytes_add_usize(&mut bytes, 1);
                bytes_add_string(&mut bytes, word);
            }
            Request::Retrieve { id } => {
                bytes_add_usize(&mut bytes, 2);
                bytes_add_usize(&mut bytes, id.clone());
            }
        };
        bytes
    }
    // Read a request from `reader` and return it. Calling `to_bytes` from above and then calling
    // `from_bytes` should return the original request. If the request is invalid, return `None`.
    pub fn from_bytes<R: std::io::Read>(mut reader: R) -> Option<Self> {
        let tag = read_usize(&mut reader)?;
        if tag == 0 {
            let doc = read_string(&mut reader)?;
            Some(Request::Publish { doc })
        } else if tag == 1 {
            let word = read_string(&mut reader)?;
            Some(Request::Search { word })
        } else if tag == 2 {
            let id = read_usize(&mut reader)?;
            Some(Request::Retrieve { id })
        } else {
            None
        }
    }
}

/// A response from the server to the client
#[derive(Debug, PartialEq)]
pub enum Response {
    /// The document was successfully added to the archive with the given index
    PublishSuccess(usize),
    /// The search for the word was successful, and the indices of the documents containing the
    /// word are returned
    SearchSuccess(Vec<usize>),
    /// The retrieval of the document was successful, and the document is returned
    RetrieveSuccess(String),
    /// The request failed
    Failure,
}
impl Response {
    // Convert the request `self` into a byte vector. See the assignment handout for suggestions on
    // how to represent the request as a series of bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            Response::PublishSuccess(n) => {
                bytes_add_usize(&mut bytes, 0);
                bytes_add_usize(&mut bytes, *n);
            }
            Response::SearchSuccess(is) => {
                bytes_add_usize(&mut bytes, 1);
                bytes_add_usize(&mut bytes, is.len());
                for i in is {
                    bytes_add_usize(&mut bytes, *i);
                }
            }
            Response::RetrieveSuccess(doc) => {
                bytes_add_usize(&mut bytes, 2);
                bytes_add_string(&mut bytes, doc);
            }
            Response::Failure => {
                bytes_add_usize(&mut bytes, 3);
            }
        };
        bytes
    }
    // Read a request from `reader` and return it. Calling `to_bytes` from above and then calling
    // `from_bytes` should return the original request. If the request is invalid, return `None`.
    pub fn from_bytes<R: std::io::Read>(mut reader: R) -> Option<Self> {
        let tag = read_usize(&mut reader)?;
        if tag == 0 {
            let id = read_usize(&mut reader)?;
            Some(Response::PublishSuccess(id))
        } else if tag == 1 {
            let is = read_vec(&mut reader)?;
            Some(Response::SearchSuccess(is))
        } else if tag == 2 {
            let doc = read_string(&mut reader)?;
            Some(Response::RetrieveSuccess(doc))
        } else if tag == 3 {
            Some(Response::Failure)
        } else {
            None
        }
    }
}

fn bytes_add_usize(bytes: &mut Vec<u8>, n: usize) {
    bytes.extend(n.to_be_bytes().iter());
}

fn bytes_add_string(bytes: &mut Vec<u8>, s: &str) {
    bytes_add_usize(bytes, s.len());
    bytes.extend(s.as_bytes());
}

fn read_usize<R: std::io::Read>(reader: &mut R) -> Option<usize> {
    let mut buf = [0; 8];
    reader.read_exact(&mut buf).ok()?;
    Some(usize::from_be_bytes(buf))
}

fn read_string<R: std::io::Read>(reader: &mut R) -> Option<String> {
    let mut len_buf = [0; 8];
    reader.read_exact(&mut len_buf).ok()?;
    let len = usize::from_be_bytes(len_buf);
    let mut buf = vec![0; len];
    reader.read_exact(&mut buf).ok()?;
    String::from_utf8(buf).ok()
}

fn read_vec<R: std::io::Read>(reader: &mut R) -> Option<Vec<usize>> {
    let mut buf = [0; 8];
    reader.read_exact(&mut buf).ok()?;
    let len = usize::from_be_bytes(buf);
    let mut vec = Vec::new();
    for _ in 0..len {
        reader.read_exact(&mut buf).ok()?;
        vec.push(usize::from_be_bytes(buf));
    }
    Some(vec)
}
