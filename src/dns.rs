use crate::RecordType;

// https://datatracker.ietf.org/doc/html/rfc1035#autoid-41:~:text=RCODE,-Response
#[derive(Debug)]
enum ResponseCode {
    NoError,        // 0
    FormatError,    // 1
    ServerFailure,  // 2
    NameError,      // 3
    NotImplemented, // 4
    Refused,        // 5
}

// https://datatracker.ietf.org/doc/html/rfc1035#autoid-41:~:text=OPCODE,-A
#[derive(Debug)]
enum Opcode {
    Query,  // 0
    IQuery, // 1
    Status, // 2
}

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
///                                 1  1  1  1  1  1
///   0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                      ID                       |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE   |
///  +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    QDCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    ANCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    NSCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    ARCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
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
    opcode: Opcode,

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
    rcode: ResponseCode,

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

/// https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.2
///                               1  1  1  1  1  1
/// 0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                                               |
/// /                     QNAME                     /
/// /                                               /
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                     QTYPE                     |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                     QCLASS                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
#[derive(Debug)]
struct DnsQuestion {
    /// A domain name represented as a sequence of labels, where
    /// each label consists of a length octet followed by that
    /// number of octets.  The domain name terminates with the
    /// zero length octet for the null label of the root.  Note
    /// that this field may be an odd number of octets; no
    /// padding is used.
    qname: String,

    /// A two octet code which specifies the type of the query.
    /// The values for this field include all codes valid for a
    /// TYPE field, together with some more general codes which
    /// can match more than one type of RR.
    qtype: u16,

    /// A two octet code that specifies the class of the query.
    /// For example, the QCLASS field is IN for the Internet.
    qclass: u16,
}

/// https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.3
///                               1  1  1  1  1  1
/// 0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                                               |
/// /                                               /
/// /                      NAME                     /
/// |                                               |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                      TYPE                     |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                     CLASS                     |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                      TTL                      |
/// |                                               |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                   RDLENGTH                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--|
/// /                     RDATA                     /
/// /                                               /
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
#[derive(Debug)]
struct ResourceRecord {
    /// A domain name to which this resource record pertains.
    name: String,

    /// two octets containing one of the RR type codes. This
    /// field specifies the meaning of the data in the RDATA field.
    r#type: RecordType,

    /// two octets which specify the class of the data in the
    /// RDATA field.
    class: u16,

    /// A 32 bit unsigned integer that specifies the time
    /// interval (in seconds) that the resource record may be
    /// cached before it should be discarded.  Zero values are
    /// interpreted to mean that the RR can only be used for the
    /// transaction in progress, and should not be cached.
    ttl: u32,

    /// An unsigned 16 bit integer that specifies the length in
    /// octets of the RDATA field.
    rdlength: u16,

    /// A variable length string of octets that describes the
    /// resource.  The format of this information varies
    /// according to the TYPE and CLASS of the resource record.
    /// For example, the if the TYPE is A and the CLASS is IN,
    /// the RDATA field is a 4 octet ARPA Internet address.
    rdata: Vec<u8>,
}
