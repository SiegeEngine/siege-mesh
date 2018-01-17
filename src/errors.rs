
error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    links {
    }

    foreign_links {
        Io(::std::io::Error);
        Bincode(::bincode::Error);
    }

    errors {
        General(s: String) {
            description("General Error"),
            display("General Error: '{}'", s),
        }
        ShortFile {
            description("Short File"),
        }
        BadMagicNumber {
            description("Bad Magic Number"),
        }
        UnknownVertexType {
            description("Unknown or Unsupported Vertex Type"),
        }
    }
}
