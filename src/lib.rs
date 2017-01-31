use std::str::FromStr;
use std::fmt;
//use std::num::ParseIntError;

enum SdpParserResult {
    ParsedSuccessfully,
    ParserLineError   { message: String,
                        line: String },
    ParserUnsupported { message: String,
                        line: String },
}

struct SdpAttribute {
    name: String,
    value: String
}

struct SdpBandwidth {
    bwtype: String,
    bandwidth: u64
}

enum SdpNetType {
    SdpNetTypeIn
}

impl fmt::Display for SdpNetType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IN")
    }
}

struct SdpConnection {
    nettype: SdpNetType,
    addrtype: String, // TODO replace with enum
    unicast_addr: String // TODO store the parsed addr
}

enum SdpMediaValue {
    SdpMediaAudio,
    SdpMediaVideo,
    SdpMediaApplication
}

impl fmt::Display for SdpMediaValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            SdpMediaValue::SdpMediaAudio       => "Audio",
            SdpMediaValue::SdpMediaVideo       => "Video",
            SdpMediaValue::SdpMediaApplication => "Application"
        };
        write!(f, "{}", printable)
    }
}

enum SdpProtocolValue {
    SdpProtoUdpTlsRtpSavpf,
    SdpProtoTcpTlsRtpSavpf,
    SdpProtoDtlsSctp,
    SdpProtoUdpDtlsSctp,
    SdpProtoTcpDtlsSctp
}

impl fmt::Display for SdpProtocolValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            SdpProtocolValue::SdpProtoUdpTlsRtpSavpf => "Udp/Tls/Rtp/Savpf",
            SdpProtocolValue::SdpProtoTcpTlsRtpSavpf => "Tcp/Tls/Rtp/Savpf",
            SdpProtocolValue::SdpProtoDtlsSctp       => "Dtls/Sctp",
            SdpProtocolValue::SdpProtoUdpDtlsSctp    => "Udp/Dtls/Sctp",
            SdpProtocolValue::SdpProtoTcpDtlsSctp    => "Tcp/Dtls/Sctp"
        };
        write!(f, "{}", printable)
    }
}

enum SdpFormatList {
    SdpFormatIntegers {list: Vec<u32>},
    SdpFormatStrings {list: Vec<String>}
}

impl fmt::Display for SdpFormatList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SdpFormatList::SdpFormatIntegers { list: ref x } => write!(f, "{:?}", x),
            SdpFormatList::SdpFormatStrings { list: ref x } => write!(f, "{:?}", x)
        }
    }
}

struct SdpMedia {
    media: SdpMediaValue,
    port: u32,
    proto: SdpProtocolValue,
    formats: SdpFormatList
}

struct SdpOrigin {
    username: String,
    session_id: u64,
    session_version: u64,
    nettype: SdpNetType,
    addrtype: String, // TODO replace with enum
    unicast_addr: String // TODO store the parsed addr
}

struct SdpTiming {
    start: u64,
    stop: u64
}

enum SdpLine {
    Attribute {attribute: SdpAttribute},
    Bandwidth {bandwidth: SdpBandwidth},
    Connection {connection: SdpConnection},
    Media {media: SdpMedia},
    Origin {origin: SdpOrigin},
    SdpString {string: String},
    SdpUInt {uint: u64},
    Timing {timing: SdpTiming}
}

/*
TODO this would allow all .parse() calls to be called from within try!() instead
of match() calls. But that requires that all functions return Result<res, error>
instead of the simple SdpParserResult right now.

https://ruudvanasseldonk.com/2015/06/17/exceptional-results-error-handling-in-csharp-and-rust

impl From<ParseIntError> for SdpParserResult {
    fn from(_: ParseIntError) -> SdpParserResult {
        SdpParserResult::ParserError { message: "parse to convert integer".to_string() }
    }
}
*/

fn create_sdp_string(value: &str) -> SdpLine {
    return SdpLine::SdpString {string: String::from(value)}
}

fn parse_repeat(value: &str) -> Result<SdpLine, SdpParserResult> {
    // TODO implement this if it's ever needed
    println!("repeat: {}", value);
    return Result::Ok(create_sdp_string(value))
}

fn parse_zone(value: &str) -> Result<SdpLine, SdpParserResult> {
    // TODO implement this if it's ever needed
    println!("zone: {}", value);
    return Result::Ok(create_sdp_string(value))
}

fn parse_key(value: &str) -> Result<SdpLine, SdpParserResult> {
    // TODO implement this if it's ever needed
    println!("key: {}", value);
    return Result::Ok(create_sdp_string(value))
}

fn parse_information(value: &str) -> Result<SdpLine, SdpParserResult> {
    println!("information: {}", value);
    return Result::Ok(create_sdp_string(value))
}

fn parse_uri(value: &str) -> Result<SdpLine, SdpParserResult> {
    // TODO check if this is really a URI
    println!("uri: {}", value);
    return Result::Ok(create_sdp_string(value))
}

fn parse_email(value: &str) -> Result<SdpLine, SdpParserResult> {
    // TODO check if this is really an email address
    println!("email: {}", value);
    return Result::Ok(create_sdp_string(value))
}

fn parse_phone(value: &str) -> Result<SdpLine, SdpParserResult> {
    // TODO check if this is really a phone number
    println!("phone: {}", value);
    return Result::Ok(create_sdp_string(value))
}

fn parse_session(value: &str) -> Result<SdpLine, SdpParserResult> {
    println!("session: {}", value);
    return Result::Ok(create_sdp_string(value))
}

fn parse_version(value: &str) -> Result<SdpLine, SdpParserResult> {
    let ver = match value.parse::<u64>() {
        Ok(n) => n,
        Err(_) => return Result::Err(SdpParserResult::ParserLineError {
            message: "failed to parse v field attribute".to_string(),
            line: value.to_string() })
    };
    if ver != 0 {
        return Result::Err(SdpParserResult::ParserLineError {
            message: "unsupported version in v field".to_string(),
            line: value.to_string() });
    };
    println!("version: {}", ver);
    let l = SdpLine::SdpUInt {uint: ver };
    return Result::Ok(l)
}

fn parse_nettype(value: &str) -> Result<SdpNetType, SdpParserResult> {
    if value.to_uppercase() != String::from("IN") {
        return Result::Err(SdpParserResult::ParserLineError {
            message: "nettype needs to be IN".to_string(),
            line: value.to_string() });
    };
    Result::Ok(SdpNetType::SdpNetTypeIn)
}

fn parse_origin(value: &str) -> Result<SdpLine, SdpParserResult> {
    let ot: Vec<&str> = value.split_whitespace().collect();
    if ot.len() != 6 {
        return Result::Err(SdpParserResult::ParserLineError {
            message: "origin field must have six tokens".to_string(),
            line: value.to_string() });
    }
    let username = ot[0];
    let session_id = match ot[1].parse::<u64>() {
        Ok(n) => n,
        Err(_) => return Result::Err(SdpParserResult::ParserLineError {
            message: "failed to parse origin session id attribute".to_string(),
            line: value.to_string() })
    };
    let session_version = match ot[2].parse::<u64>() {
        Ok(n) => n,
        Err(_) => return Result::Err(SdpParserResult::ParserLineError {
            message: "failed to parse origin session version attribute".to_string(),
            line: value.to_string() })
    };
    let nettype = match parse_nettype(ot[3]) {
        Ok(n) => n,
        Err(e) => { return Result::Err(e) }
    };
    let addrtype = ot[4];
    let unicast_addr = ot[5];
    match addrtype.to_uppercase().as_ref() {
        "IP4" => {
            match std::net::Ipv4Addr::from_str(unicast_addr) {
                Ok(n) => n,
                Err(_) => return Result::Err(SdpParserResult::ParserLineError {
                    message: "failed to parse origin unicast IP4 address attribute".to_string(),
                    line: value.to_string() })
            };
        },
        "IP6" => {
            match std::net::Ipv6Addr::from_str(unicast_addr) {
                Ok(n) => n,
                Err(_) => return Result::Err(SdpParserResult::ParserLineError {
                    message: "failed to parse origin unicast IP6 address attribute".to_string(),
                    line: value.to_string() })
            };
        },
        _ => return Result::Err(SdpParserResult::ParserLineError {
            message: "address type in origin needs to be IP4 or IP6".to_string(),
            line: value.to_string() })
    }
    let o = SdpOrigin { username: String::from(username),
                        session_id: session_id,
                        session_version: session_version,
                        nettype: nettype,
                        addrtype: String::from(addrtype),
                        unicast_addr: String::from(unicast_addr) };
    println!("origin: {}, {}, {}, {}, {}, {}",
             o.username, o.session_id, o.session_version, o.nettype,
             o.addrtype, o.unicast_addr);
    let l = SdpLine::Origin { origin: o };
    return Result::Ok(l)
}

fn parse_connection(value: &str) -> Result<SdpLine, SdpParserResult> {
    let cv: Vec<&str> = value.split_whitespace().collect();
    if cv.len() != 3 {
        return Result::Err(SdpParserResult::ParserLineError {
            message: "connection attribute must have three tokens".to_string(),
            line: value.to_string() });
    }
    // TODO this is exactly the same parser as the end of origin.
    //      Share it in a function?!
    let nettype = match parse_nettype(cv[0]) {
        Ok(n) => n,
        Err(e) => { return Result::Err(e) }
    };
    let addrtype = cv[1];
    let unicast_addr = cv[2];
    match addrtype.to_uppercase().as_ref() {
        "IP4" => {
            match std::net::Ipv4Addr::from_str(unicast_addr) {
                Ok(n) => n,
                Err(_) => return Result::Err(SdpParserResult::ParserLineError {
                    message: "failed to parse connection IP4 address attribute".to_string(),
                    line: value.to_string() })
            };
        },
        "IP6" => {
            match std::net::Ipv6Addr::from_str(unicast_addr) {
                Ok(n) => n,
                Err(_) => return Result::Err(SdpParserResult::ParserLineError {
                    message: "failed to parse connection IP6 address attribute".to_string(),
                    line: value.to_string() })
            };
        },
        _ => return Result::Err(SdpParserResult::ParserLineError {
            message: "address type in connection needs to be IP4 or IP6".to_string(),
            line: value.to_string() })
    }
    let c = SdpConnection { nettype: nettype,
                            addrtype: String::from(addrtype),
                            unicast_addr: String::from(unicast_addr) };
    println!("connection: {}, {}, {}",
             c.nettype, c.addrtype, c.unicast_addr);
    let l = SdpLine::Connection { connection: c };
    return Result::Ok(l)
}

fn parse_bandwidth(value: &str) -> Result<SdpLine, SdpParserResult> {
    let bv: Vec<&str> = value.split(':').collect();
    if bv.len() != 2 {
        return Result::Err(SdpParserResult::ParserLineError {
            message: "bandwidth attribute must have two tokens".to_string(),
            line: value.to_string() });
    }
    let bwtype = bv[0]; // TODO check for supported values
    match bwtype.to_uppercase().as_ref() {
        "AS" | "TIAS" => (),
        _ => return Result::Err(SdpParserResult::ParserUnsupported {
              message: "unsupported bandwidth type value".to_string(),
              line: value.to_string() }),
    }
    let bandwidth = match bv[1].parse::<u64>() {
        Ok(n) => n,
        Err(_) => return Result::Err(SdpParserResult::ParserLineError {
            message: "failed to parse bandwidth number attribute".to_string(),
            line: value.to_string() })
    };
    let b = SdpBandwidth { bwtype: String::from(bwtype),
                            bandwidth: bandwidth };
    println!("bandwidth: {}, {}",
             b.bwtype, b.bandwidth);
    let l = SdpLine::Bandwidth { bandwidth: b };
    return Result::Ok(l)
}

fn parse_timing(value: &str) -> Result<SdpLine, SdpParserResult> {
    let tv: Vec<&str> = value.split_whitespace().collect();
    if tv.len() != 2 {
        return Result::Err(SdpParserResult::ParserLineError {
            message: "timing attribute must have two tokens".to_string(),
            line: value.to_string() });
    }
    let start_time = match tv[0].parse::<u64>() {
        Ok(n) => n,
        Err(_) => return Result::Err(SdpParserResult::ParserLineError {
            message: "failed to parse timing start time attribute".to_string(),
            line: value.to_string() })
    };
    let stop_time = match tv[1].parse::<u64>() {
        Ok(n) => n,
        Err(_) => return Result::Err(SdpParserResult::ParserLineError {
            message: "failed to parse timing stop time attribute".to_string(),
            line: value.to_string() })
    };
    let t = SdpTiming { start: start_time,
                        stop: stop_time };
    println!("timing: {}, {}", t.start, t.stop);
    let l = SdpLine::Timing {timing: t};
    return Result::Ok(l)
}

fn parse_media_token(value: &str) -> Result<SdpMediaValue, SdpParserResult> {
    let media = match value.to_lowercase().as_ref() {
        "audio"       => SdpMediaValue::SdpMediaAudio,
        "video"       => SdpMediaValue::SdpMediaVideo,
        "application" => SdpMediaValue::SdpMediaApplication,
        _ => return Result::Err(SdpParserResult::ParserUnsupported {
              message: "unsupported media value".to_string(),
              line: value.to_string() }),
    };
    Result::Ok(media)
}

fn parse_protocol_token(value: &str) -> Result<SdpProtocolValue, SdpParserResult> {
    let proto = match value.to_uppercase().as_ref() {
        "UDP/TLS/RTP/SAVPF" => SdpProtocolValue::SdpProtoUdpTlsRtpSavpf,
        "TCP/TLS/RTP/SAVPF" => SdpProtocolValue::SdpProtoTcpTlsRtpSavpf,
        "DTLS/SCTP"         => SdpProtocolValue::SdpProtoDtlsSctp,
        "UDP/DTLS/SCTP"     => SdpProtocolValue::SdpProtoUdpDtlsSctp,
        "TCP/DTLS/SCTP"     => SdpProtocolValue::SdpProtoTcpDtlsSctp,
        _ => return Result::Err(SdpParserResult::ParserUnsupported {
              message: "unsupported protocol value".to_string(),
              line: value.to_string() }),
    };
    Result::Ok(proto)
}

fn parse_media(value: &str) -> Result<SdpLine, SdpParserResult> {
    let mv: Vec<&str> = value.split_whitespace().collect();
    if mv.len() < 4 {
        return Result::Err(SdpParserResult::ParserLineError {
            message: "media attribute must have at least four tokens".to_string(),
            line: value.to_string() });
    }
    let media_str = mv[0];
    let media = match parse_media_token(media_str) {
        Ok(n) => n,
        Err(e) => { return Result::Err(e) }
    };
    let port = match mv[1].parse::<u32>() {
        Ok(n) => n,
        Err(_) => return Result::Err(SdpParserResult::ParserLineError {
            message: "failed to parse media port token".to_string(),
            line: value.to_string() })
    };
    if port > 65535 {
        return Result::Err(SdpParserResult::ParserLineError {
            message: "media port token is too big".to_string(),
            line: value.to_string() })
    }
    let proto_str = mv[2];
    let proto = match parse_protocol_token(proto_str) {
        Ok(n) => n,
        Err(e) => { return Result::Err(e) }
    };
    let fmt_slice: &[&str] = &mv[3..];
    let fmt = match media {
        SdpMediaValue::SdpMediaAudio | SdpMediaValue::SdpMediaVideo => {
            let mut fmt_vec: Vec<u32> = vec![];
            for num in fmt_slice {
                let fmt_num = match num.parse::<u32>() {
                    Ok(n) => (n),
                    Err(_) => return Result::Err(SdpParserResult::ParserLineError {
                        message: "failed to parse format number in media line".to_string(),
                        line: value.to_string() }),
                };
                match fmt_num {
                    0 => (),           // PCMU
                    8 => (),           // PCMA
                    9 => (),           // G722
                    13 => (),          // Comfort Noise
                    96 ... 127 => (),  // dynamic range
                    _ => return Result::Err(SdpParserResult::ParserLineError {
                          message: "format number in media line is out of range".to_string(),
                          line: value.to_string() }),
                };
                fmt_vec.push(fmt_num);
            };
            SdpFormatList::SdpFormatIntegers { list: fmt_vec }
        },
        SdpMediaValue::SdpMediaApplication => {
            let mut fmt_vec: Vec<String> = vec![];
            // TODO enforce length == 1 and content 'webrtc-datachannel' only?
            for token in fmt_slice {
                fmt_vec.push(String::from(*token));
            }
            SdpFormatList::SdpFormatStrings { list: fmt_vec }
        }
    };
    let m = SdpMedia { media: media,
                       port: port,
                       proto: proto,
                       formats: fmt };
    println!("media: {}, {}, {}, {}",
             m.media, m.port, m.proto, m.formats);
    let l = SdpLine::Media { media: m };
    return Result::Ok(l)
}

fn parse_attribute(value: &str) -> Result<SdpLine, SdpParserResult> {
    let attribute = value;
    let colon = attribute.find(':');
    let name: &str;
    let mut value: &str = "";
    if colon == None {
        name = attribute;
    } else {
        let (aname, avalue) = attribute.split_at(colon.unwrap());
        name = aname;
        value = avalue;
    }
    match name.to_lowercase().as_ref() {
        // TODO TODO TODO
        "recvonly" => (),
        "sendonly" => (),
        "inactive" => (),
        "sendrecv" => (),
        "ssrc" => (),
        "ssrc-group" => (),
        "rtpmap" => (),
        "fmtp" => (),
        "rtcp" => (),
        "rtcp-fb" => (),
        "rtcp-mux" => (),
        "rtcp-rsize" => (),
        "msid" => (),
        "msid-semantic" => (),
        "mid" => (),
        "ice-ufrag" => (),
        "ice-pwd" => (),
        "ice-options" => (),
        "candidate" => (),
        "end-of-candidates" => (),
        "setup" => (),
        "extmap" => (),
        "group" => (),
        "fingerprint" => (),
        "sctpmap" => (),
        _ => return Result::Err(SdpParserResult::ParserUnsupported {
              message: "unsupported attribute value".to_string(),
              line: name.to_string() }),
    }
    let a = SdpAttribute { name: String::from(name),
                           value: String::from(value) };
    println!("attribute: {}, {}", 
             a.name, a.value);
    let l = SdpLine::Attribute { attribute: a };
    return Result::Ok(l)
}

fn parse_sdp_line(line: &str) -> SdpParserResult {
    let v: Vec<&str> = line.splitn(2, '=').collect();
    if v.len() < 2 {
        return SdpParserResult::ParserLineError {
            message: "failed to split field and attribute".to_string(),
            line: line.to_string() };
    };
    let name = v[0].trim();
    if name.is_empty() || name.len() > 1 {
        return SdpParserResult::ParserLineError {
            message: "field name empty or too long".to_string(),
            line: line.to_string() };
    };
    let value = v[1].trim();
    if value.len() == 0 {
        return SdpParserResult::ParserLineError {
            message: "attribute value has zero length".to_string(),
            line: line.to_string() };
    }
    let line = match name.to_lowercase().as_ref() {
        "a" => { parse_attribute(value) },
        "b" => { parse_bandwidth(value) },
        "c" => { parse_connection(value) },
        "e" => { parse_email(value) },
        "i" => { parse_information(value) },
        "k" => { parse_key(value) },
        "m" => { parse_media(value) },
        "o" => { parse_origin(value) },
        "p" => { parse_phone(value) },
        "r" => { parse_repeat(value) },
        "s" => { parse_session(value) },
        "t" => { parse_timing(value) },
        "u" => { parse_uri(value) },
        "v" => { parse_version(value) },
        "z" => { parse_zone(value) },
        _   => { return SdpParserResult::ParserLineError {
                    message: "unsupported sdp field".to_string(),
                    line: line.to_string() } }
    };
    // TODO there must be a way to error right from the previous match
    match line {
        Ok(n) => { println!("parsed successfully") },
        Err(e) => { return e }
    }
    return SdpParserResult::ParsedSuccessfully
}

pub fn parse_sdp(sdp: &str, fail_on_warning: bool) -> bool {
    if sdp.is_empty() {
        return false;
    }
    let lines = sdp.lines();
    let mut v: Vec<SdpParserResult> = Vec::new();
    for line in lines {
        let result = parse_sdp_line(line);
        match result {
            SdpParserResult::ParsedSuccessfully => (),
            // FIXME is this really a good way to accomplish this?
            SdpParserResult::ParserLineError { message: x, line: y } =>
                { v.push(SdpParserResult::ParserLineError { message: x, line: y}) }
            SdpParserResult::ParserUnsupported { message: x, line: y } =>
                {
                    if fail_on_warning {
                        v.push(SdpParserResult::ParserUnsupported { message: x, line: y});
                    } else {
                        println!("Warning unsupported value encountered: {}\n in line {}", x, y);
                    }
                }
        };
    };
    if v.len() > 0 {
        while let Some(x) = v.pop() {
            match x {
                SdpParserResult::ParsedSuccessfully => {}, // TODO should we fail here?
                SdpParserResult::ParserLineError { message: msg, line: l} =>
                    { println!("Parser error: {}\n  in line: {}", msg, l) }
                SdpParserResult::ParserUnsupported { message: msg, line: l} =>
                    { println!("Parser unknown: {}\n  in line: {}", msg, l) }
            }
        }
        return false;
    };
    true
}
