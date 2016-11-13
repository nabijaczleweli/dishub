(function() {var implementors = {};
implementors["lazy_static"] = [];implementors["libc"] = [];implementors["regex_syntax"] = [];implementors["num"] = [];implementors["phf"] = [];implementors["flate2"] = ["impl&lt;W:&nbsp;<a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='flate2/write/struct.DeflateEncoder.html' title='flate2::write::DeflateEncoder'>EncoderWriter</a>&lt;W&gt;","impl&lt;W:&nbsp;<a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='flate2/write/struct.DeflateDecoder.html' title='flate2::write::DeflateDecoder'>DecoderWriter</a>&lt;W&gt;","impl&lt;W:&nbsp;<a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='flate2/write/struct.GzEncoder.html' title='flate2::write::GzEncoder'>EncoderWriter</a>&lt;W&gt;","impl&lt;W:&nbsp;<a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='flate2/write/struct.ZlibEncoder.html' title='flate2::write::ZlibEncoder'>EncoderWriter</a>&lt;W&gt;","impl&lt;W:&nbsp;<a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='flate2/write/struct.ZlibDecoder.html' title='flate2::write::ZlibDecoder'>DecoderWriter</a>&lt;W&gt;",];implementors["serde"] = [];implementors["openssl"] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='openssl/bio/struct.MemBio.html' title='openssl::bio::MemBio'>MemBio</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='openssl/crypto/hash/struct.Hasher.html' title='openssl::crypto::hash::Hasher'>Hasher</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='openssl/crypto/hmac/struct.HMAC.html' title='openssl::crypto::hmac::HMAC'>HMAC</a>","impl&lt;S:&nbsp;<a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> + <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='openssl/ssl/struct.SslStream.html' title='openssl::ssl::SslStream'>SslStream</a>&lt;S&gt;","impl&lt;S&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='enum' href='openssl/ssl/enum.MaybeSslStream.html' title='openssl::ssl::MaybeSslStream'>MaybeSslStream</a>&lt;S&gt; <span class='where'>where S: <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> + <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a></span>",];implementors["mime_guess"] = [];implementors["chrono"] = [];implementors["hyper"] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='openssl/bio/struct.MemBio.html' title='openssl::bio::MemBio'>MemBio</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='openssl/crypto/hash/struct.Hasher.html' title='openssl::crypto::hash::Hasher'>Hasher</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='openssl/crypto/hmac/struct.HMAC.html' title='openssl::crypto::hmac::HMAC'>HMAC</a>","impl&lt;S&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='openssl/ssl/struct.SslStream.html' title='openssl::ssl::SslStream'>SslStream</a>&lt;S&gt; <span class='where'>where S: <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> + <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a></span>","impl&lt;S&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='enum' href='openssl/ssl/enum.MaybeSslStream.html' title='openssl::ssl::MaybeSslStream'>MaybeSslStream</a>&lt;S&gt; <span class='where'>where S: <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> + <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a></span>","impl&lt;S:&nbsp;<a class='trait' href='hyper/net/trait.NetworkStream.html' title='hyper::net::NetworkStream'>NetworkStream</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='hyper/client/pool/struct.PooledStream.html' title='hyper::client::pool::PooledStream'>PooledStream</a>&lt;S&gt;","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='hyper/client/request/struct.Request.html' title='hyper::client::request::Request'>Request</a>&lt;<a class='enum' href='hyper/net/enum.Streaming.html' title='hyper::net::Streaming'>Streaming</a>&gt;","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='hyper/http/h1/struct.Http11Message.html' title='hyper::http::h1::Http11Message'>Http11Message</a>","impl&lt;W:&nbsp;<a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='enum' href='hyper/http/h1/enum.HttpWriter.html' title='hyper::http::h1::HttpWriter'>HttpWriter</a>&lt;W&gt;","impl&lt;S&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='hyper/http/h2/struct.Http2Message.html' title='hyper::http::h2::Http2Message'>Http2Message</a>&lt;S&gt; <span class='where'>where S: <a class='trait' href='hyper/http/h2/trait.CloneableStream.html' title='hyper::http::h2::CloneableStream'>CloneableStream</a></span>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='hyper/net/struct.HttpStream.html' title='hyper::net::HttpStream'>HttpStream</a>","impl&lt;S:&nbsp;<a class='trait' href='hyper/net/trait.NetworkStream.html' title='hyper::net::NetworkStream'>NetworkStream</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='enum' href='hyper/net/enum.HttpsStream.html' title='hyper::net::HttpsStream'>HttpsStream</a>&lt;S&gt;","impl&lt;'a&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='hyper/server/response/struct.Response.html' title='hyper::server::response::Response'>Response</a>&lt;'a,&nbsp;<a class='enum' href='hyper/net/enum.Streaming.html' title='hyper::net::Streaming'>Streaming</a>&gt;",];implementors["multipart"] = ["impl&lt;R&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='multipart/client/struct.SizedRequest.html' title='multipart::client::SizedRequest'>SizedRequest</a>&lt;R&gt;",];implementors["websocket"] = ["impl&lt;'w,&nbsp;W&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='websocket/ws/util/mask/struct.Masker.html' title='websocket::ws::util::mask::Masker'>Masker</a>&lt;'w,&nbsp;W&gt; <span class='where'>where W: <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> + 'w</span>","impl&lt;S&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='hyper/client/pool/struct.PooledStream.html' title='hyper::client::pool::PooledStream'>PooledStream</a>&lt;S&gt; <span class='where'>where S: <a class='trait' href='hyper/net/trait.NetworkStream.html' title='hyper::net::NetworkStream'>NetworkStream</a></span>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='hyper/client/request/struct.Request.html' title='hyper::client::request::Request'>Request</a>&lt;<a class='enum' href='hyper/net/enum.Streaming.html' title='hyper::net::Streaming'>Streaming</a>&gt;","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='hyper/http/h1/struct.Http11Message.html' title='hyper::http::h1::Http11Message'>Http11Message</a>","impl&lt;W&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='enum' href='hyper/http/h1/enum.HttpWriter.html' title='hyper::http::h1::HttpWriter'>HttpWriter</a>&lt;W&gt; <span class='where'>where W: <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a></span>","impl&lt;S&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='hyper/http/h2/struct.Http2Message.html' title='hyper::http::h2::Http2Message'>Http2Message</a>&lt;S&gt; <span class='where'>where S: <a class='trait' href='hyper/http/h2/trait.CloneableStream.html' title='hyper::http::h2::CloneableStream'>CloneableStream</a></span>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='hyper/net/struct.HttpStream.html' title='hyper::net::HttpStream'>HttpStream</a>","impl&lt;S&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='enum' href='hyper/net/enum.HttpsStream.html' title='hyper::net::HttpsStream'>HttpsStream</a>&lt;S&gt; <span class='where'>where S: <a class='trait' href='hyper/net/trait.NetworkStream.html' title='hyper::net::NetworkStream'>NetworkStream</a></span>","impl&lt;'a&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='hyper/server/response/struct.Response.html' title='hyper::server::response::Response'>Response</a>&lt;'a,&nbsp;<a class='enum' href='hyper/net/enum.Streaming.html' title='hyper::net::Streaming'>Streaming</a>&gt;","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='openssl/bio/struct.MemBio.html' title='openssl::bio::MemBio'>MemBio</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='openssl/crypto/hash/struct.Hasher.html' title='openssl::crypto::hash::Hasher'>Hasher</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='openssl/crypto/hmac/struct.HMAC.html' title='openssl::crypto::hmac::HMAC'>HMAC</a>","impl&lt;S&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='openssl/ssl/struct.SslStream.html' title='openssl::ssl::SslStream'>SslStream</a>&lt;S&gt; <span class='where'>where S: <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> + <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a></span>","impl&lt;S&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='enum' href='openssl/ssl/enum.MaybeSslStream.html' title='openssl::ssl::MaybeSslStream'>MaybeSslStream</a>&lt;S&gt; <span class='where'>where S: <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> + <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a></span>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='enum' href='websocket/stream/enum.WebSocketStream.html' title='websocket::stream::WebSocketStream'>WebSocketStream</a>",];implementors["dishub"] = ["impl&lt;S&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='hyper/client/pool/struct.PooledStream.html' title='hyper::client::pool::PooledStream'>PooledStream</a>&lt;S&gt; <span class='where'>where S: <a class='trait' href='hyper/net/trait.NetworkStream.html' title='hyper::net::NetworkStream'>NetworkStream</a></span>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='hyper/client/request/struct.Request.html' title='hyper::client::request::Request'>Request</a>&lt;<a class='enum' href='hyper/net/enum.Streaming.html' title='hyper::net::Streaming'>Streaming</a>&gt;","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='hyper/http/h1/struct.Http11Message.html' title='hyper::http::h1::Http11Message'>Http11Message</a>","impl&lt;W&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='enum' href='hyper/http/h1/enum.HttpWriter.html' title='hyper::http::h1::HttpWriter'>HttpWriter</a>&lt;W&gt; <span class='where'>where W: <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a></span>","impl&lt;S&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='hyper/http/h2/struct.Http2Message.html' title='hyper::http::h2::Http2Message'>Http2Message</a>&lt;S&gt; <span class='where'>where S: <a class='trait' href='hyper/http/h2/trait.CloneableStream.html' title='hyper::http::h2::CloneableStream'>CloneableStream</a></span>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='hyper/net/struct.HttpStream.html' title='hyper::net::HttpStream'>HttpStream</a>","impl&lt;S&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='enum' href='hyper/net/enum.HttpsStream.html' title='hyper::net::HttpsStream'>HttpsStream</a>&lt;S&gt; <span class='where'>where S: <a class='trait' href='hyper/net/trait.NetworkStream.html' title='hyper::net::NetworkStream'>NetworkStream</a></span>","impl&lt;'a&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='hyper/server/response/struct.Response.html' title='hyper::server::response::Response'>Response</a>&lt;'a,&nbsp;<a class='enum' href='hyper/net/enum.Streaming.html' title='hyper::net::Streaming'>Streaming</a>&gt;","impl&lt;W&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='flate2/deflate/struct.EncoderWriter.html' title='flate2::deflate::EncoderWriter'>EncoderWriter</a>&lt;W&gt; <span class='where'>where W: <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a></span>","impl&lt;W&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='flate2/deflate/struct.DecoderWriter.html' title='flate2::deflate::DecoderWriter'>DecoderWriter</a>&lt;W&gt; <span class='where'>where W: <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a></span>","impl&lt;W&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='flate2/gz/struct.EncoderWriter.html' title='flate2::gz::EncoderWriter'>EncoderWriter</a>&lt;W&gt; <span class='where'>where W: <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a></span>","impl&lt;W&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='flate2/zlib/struct.EncoderWriter.html' title='flate2::zlib::EncoderWriter'>EncoderWriter</a>&lt;W&gt; <span class='where'>where W: <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a></span>","impl&lt;W&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='flate2/zlib/struct.DecoderWriter.html' title='flate2::zlib::DecoderWriter'>DecoderWriter</a>&lt;W&gt; <span class='where'>where W: <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a></span>","impl&lt;R&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='multipart/client/sized/struct.SizedRequest.html' title='multipart::client::sized::SizedRequest'>SizedRequest</a>&lt;R&gt;","impl&lt;'w,&nbsp;W&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='websocket/ws/util/mask/struct.Masker.html' title='websocket::ws::util::mask::Masker'>Masker</a>&lt;'w,&nbsp;W&gt; <span class='where'>where W: <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> + 'w</span>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='enum' href='websocket/stream/enum.WebSocketStream.html' title='websocket::stream::WebSocketStream'>WebSocketStream</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='openssl/bio/struct.MemBio.html' title='openssl::bio::MemBio'>MemBio</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='openssl/crypto/hash/struct.Hasher.html' title='openssl::crypto::hash::Hasher'>Hasher</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='openssl/crypto/hmac/struct.HMAC.html' title='openssl::crypto::hmac::HMAC'>HMAC</a>","impl&lt;S&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='openssl/ssl/struct.SslStream.html' title='openssl::ssl::SslStream'>SslStream</a>&lt;S&gt; <span class='where'>where S: <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> + <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a></span>","impl&lt;S&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='enum' href='openssl/ssl/enum.MaybeSslStream.html' title='openssl::ssl::MaybeSslStream'>MaybeSslStream</a>&lt;S&gt; <span class='where'>where S: <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> + <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a></span>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
