
error_chain! {
    errors {
        UnknownCompression(code: u8)
        InvalidMessageLength
        Disconnected
        AuthFailed
        UnexpectedType
        Decoding
        UnknownTag(tag: [u8; 3])
        MissingResponsePromise
    }

    foreign_links {
        Io(::std::io::Error);
    }
}
