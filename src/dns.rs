
/// https://datatracker.ietf.org/doc/html/rfc1035#section-4.1
/// RR means Resource Record
/// +---------------------+
/// |        Header       |
/// +---------------------+
/// |       Question      | the question for the name server
/// +---------------------+
/// |        Answer       | RRs answering the question
/// +---------------------+
/// |      Authority      | RRs pointing toward an authority
/// +---------------------+
/// |      Additional     | RRs holding additional information
/// +---------------------+
#[derive(Debug)]
struct DnsMessage {
    /// DNS Header
    header: DnsHeader,

    /// the question for the name server. It's a vector to match RFC, but we will almost always a length of 1
    question: Vec<DnsQuestion>,

    /// RRs answering the question
    answer: Vec<ResourceRecord>,

    /// RRs pointing toward an authority
    authority: Vec<ResourceRecord>,

    /// RRs holding additional information
    additional: Vec<ResourceRecord>,
}

/// https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.1
#[derive(Debug)]
struct DnsHeader {
    /// A 16 bit identifier assigned by the program that
    /// generates any kind of query.  This identifier is copied
    /// the corresponding reply and can be used by the requester
    /// to match up replies to outstanding queries.
    id: u16,

    /// A one bit field that specifies whether this message is a
    /// query (0), or a response (1).
    qr: bool,

    /// A four bit field that specifies kind of query in this
    /// message.  This value is set by the originator of a query
    /// and copied into the response. The values are:
    /// 0               a standard query (QUERY)
    /// 1               an inverse query (IQUERY)
    /// 2               a server status request (STATUS)
    /// 3-15            reserved for future use
    /// Note: We use u8 because there is no u4
    opcode: u8,

    /// Authoritative Answer - this bit is valid in responses,
    /// and specifies that the responding name server is an
    /// authority for the domain name in question section.
    /// Note that the contents of the answer section may have
    /// multiple owner names because of aliases.
    aa: bool,

    /// TrunCation - specifies that this message was truncated
    /// due to length greater than that permitted on the
    /// transmission channel.
    tc: bool,

    /// Recursion Desired - this bit may be set in a query and
    /// is copied into the response.  If RD is set, it directs
    /// the name server to pursue the query recursively.
    /// Recursive query support is optional.
    rd: bool,

    /// Recursion Available - this be is set or cleared in a
    /// response, and denotes whether recursive query support is
    /// available in the name server.
    ra: bool,

    /// Reserved/DNSSEC-related bits. Keep as 0 for now.
    z: u8,

    /// Response code - this 4 bit field is set as part of
    /// responses.  The values have the following
    /// interpretation:
    /// 0               No error condition
    /// 1               Format error - The name server was
    ///                 unable to interpret the query.
    /// 
    /// 2               Server failure - The name server was
    ///                 unable to process this query due to a
    ///                 problem with the name server.
    ///
    /// 3               Name Error - Meaningful only for
    ///                 responses from an authoritative name
    ///                 server, this code signifies that the
    ///                 domain name referenced in the query does
    ///                 not exist.
    ///
    /// 4               Not Implemented - The name server does
    ///                 not support the requested kind of query.
    ///
    /// 5               Refused - The name server refuses to
    ///                 perform the specified operation for
    ///                 policy reasons.  For example, a name
    ///                 server may not wish to provide the
    ///                 information to the particular requester,
    ///                 or a name server may not wish to perform
    ///                 a particular operation (e.g., zone transfer) 
    ///                 for particular data.
    ///
    /// 6-15            Reserved for future use.
    rcode: u8,

    /// An unsigned 16 bit integer specifying the number of
    /// entries in the question section.
    qdcount: u16,

    /// An unsigned 16 bit integer specifying the number of
    /// resource records in the answer section.
    ancount: u16,

    /// An unsigned 16 bit integer specifying the number of name
    /// server resource records in the authority records section.
    nscount: u16,

    /// An unsigned 16 bit integer specifying the number of
    /// resource records in the additional records section.
    arcount: u16,
}

#[derive(Debug)]
struct DnsQuestion {

}
#[derive(Debug)]
struct ResourceRecord {

}
