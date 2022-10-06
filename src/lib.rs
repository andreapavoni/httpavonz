use axum::http::HeaderMap;

pub mod web;

pub struct HttpStatusResponse<'a> {
    pub code: u16,
    pub description: &'a str,
    pub headers: Option<HeaderMap>,
    pub exclude_body: bool,
    pub body: Option<&'a str>,
}
impl HttpStatusResponse<'static> {
    pub fn new(status: u16) -> HttpStatusResponse<'static> {
        match status {
            100 => HttpStatusResponse {
                code: 100,
                description: "Continue",
                headers: None,
                exclude_body: true,
                body: None,
            },
            101 => HttpStatusResponse {
                code: 101,
                description: "Switching Protocols",
                headers: None,
                exclude_body: true,
                body: None,
            },
            102 => HttpStatusResponse {
                code: 102,
                description: "Processing",
                headers: None,
                exclude_body: true,
                body: None,
            },
            103 => {
                let mut headers = HeaderMap::new();
                headers.insert("Link", "</css/main.css>; rel=preload".parse().unwrap());
                HttpStatusResponse {
                    code: 103,
                    description: "Early Hints",
                    headers: Some(headers),
                    exclude_body: true,
                    body: None,
                }
            }
            200 => HttpStatusResponse {
                code: 200,
                description: "OK",
                headers: None,
                exclude_body: false,
                body: None,
            },
            201 => HttpStatusResponse {
                code: 201,
                description: "Created",
                headers: None,
                exclude_body: false,
                body: None,
            },
            202 => HttpStatusResponse {
                code: 202,
                description: "Accepted",
                headers: None,
                exclude_body: false,
                body: None,
            },
            203 => HttpStatusResponse {
                code: 203,
                description: "Non-Authoritative Information",
                headers: None,
                exclude_body: false,
                body: None,
            },
            204 => HttpStatusResponse {
                code: 204,
                description: "No Content",
                headers: None,
                exclude_body: true,
                body: None,
            },
            205 => HttpStatusResponse {
                code: 205,
                description: "Reset Content",
                headers: None,
                exclude_body: true,
                body: None,
            },
            206 => {
                let mut headers = HeaderMap::new();
                headers.insert("Content-Range", "0-30".parse().unwrap());
                HttpStatusResponse {
                    code: 206,
                    description: "Partial Content",
                    headers: Some(headers),
                    exclude_body: false,
                    body: None,
                }
            }
            207 => {
                let mut headers = HeaderMap::new();
                headers.insert(
                    "Content-Type",
                    "application/xml; charset=\"utf-8\"".parse().unwrap(),
                );

                let body = r#"
<?xml version=""1.0"" encoding=""utf-8"" ?>
<d:multistatus xmlns:d=""DAV:"">
  <d:response>
    <d:href>http://www.example.com/container/resource3</d:href>
    <d:status>HTTP/1.1 423 Locked</d:status>
    <d:error><d:lock-token-submitted/></d:error>
  </d:response>
</d:multistatus>
            "#;
                HttpStatusResponse {
                    code: 207,
                    description: "Multi-Status",
                    headers: Some(headers),
                    exclude_body: false,
                    body: Some(body),
                }
            }
            208 => HttpStatusResponse {
                code: 208,
                description: "Already Reported",
                headers: None,
                exclude_body: false,
                body: None,
            },
            226 => HttpStatusResponse {
                code: 226,
                description: "IM Use",
                headers: None,
                exclude_body: false,
                body: None,
            },
            300 => HttpStatusResponse {
                code: 300,
                description: "Multiple Choices",
                headers: None,
                exclude_body: false,
                body: None,
            },
            301 => {
                let mut headers = HeaderMap::new();
                headers.insert("Location", "/".parse().unwrap());
                HttpStatusResponse {
                    code: 301,
                    description: "Moved Permanently",
                    headers: Some(headers),
                    exclude_body: false,
                    body: None,
                }
            }
            302 => {
                let mut headers = HeaderMap::new();
                headers.insert("Location", "/".parse().unwrap());
                HttpStatusResponse {
                    code: 302,
                    description: "Found",
                    headers: Some(headers),
                    exclude_body: false,
                    body: None,
                }
            }
            303 => {
                let mut headers = HeaderMap::new();
                headers.insert("Location", "/".parse().unwrap());
                HttpStatusResponse {
                    code: 303,
                    description: "See Other",
                    headers: Some(headers),
                    exclude_body: false,
                    body: None,
                }
            }
            304 => HttpStatusResponse {
                code: 304,
                description: "Not Modified",
                headers: None,
                exclude_body: true,
                body: None,
            },
            305 => HttpStatusResponse {
                code: 305,
                description: "Use Proxy",
                headers: None,
                exclude_body: false,
                body: None,
            },
            306 => HttpStatusResponse {
                code: 306,
                description: "Switch Proxy",
                headers: None,
                exclude_body: false,
                body: None,
            },
            307 => {
                let mut headers = HeaderMap::new();
                headers.insert("Location", "/".parse().unwrap());
                HttpStatusResponse {
                    code: 307,
                    description: "Temporary Redirect",
                    headers: Some(headers),
                    exclude_body: false,
                    body: None,
                }
            }
            308 => {
                let mut headers = HeaderMap::new();
                headers.insert("Location", "/".parse().unwrap());
                HttpStatusResponse {
                    code: 308,
                    description: "Permanent Redirect",
                    headers: Some(headers),
                    exclude_body: false,
                    body: None,
                }
            }
            400 => HttpStatusResponse {
                code: 400,
                description: "Bad Request",
                headers: None,
                exclude_body: false,
                body: None,
            },
            401 => {
                let mut headers = HeaderMap::new();
                headers.insert(
                    "WWW-Authenticate",
                    "Basic realm=\"Fake Realm\"".parse().unwrap(),
                );
                HttpStatusResponse {
                    code: 401,
                    description: "Unauthorized",
                    headers: Some(headers),
                    exclude_body: false,
                    body: None,
                }
            }
            402 => HttpStatusResponse {
                code: 402,
                description: "Payment Required",
                headers: None,
                exclude_body: false,
                body: None,
            },
            403 => HttpStatusResponse {
                code: 403,
                description: "Forbidden",
                headers: None,
                exclude_body: false,
                body: None,
            },
            404 => HttpStatusResponse {
                code: 404,
                description: "Not Found",
                headers: None,
                exclude_body: false,
                body: None,
            },
            405 => HttpStatusResponse {
                code: 405,
                description: "Method Not Allowed",
                headers: None,
                exclude_body: false,
                body: None,
            },
            406 => HttpStatusResponse {
                code: 406,
                description: "Not Acceptable",
                headers: None,
                exclude_body: false,
                body: None,
            },
            407 => {
                let mut headers = HeaderMap::new();
                headers.insert(
                    "Proxy-Authenticate",
                    "Basic realm=\"Fake Realm\"".parse().unwrap(),
                );

                HttpStatusResponse {
                    code: 407,
                    description: "Proxy Authentication Required",
                    headers: Some(headers),
                    exclude_body: false,
                    body: None,
                }
            }
            408 => HttpStatusResponse {
                code: 408,
                description: "Request Timeout",
                headers: None,
                exclude_body: false,
                body: None,
            },
            409 => HttpStatusResponse {
                code: 409,
                description: "Conflict",
                headers: None,
                exclude_body: false,
                body: None,
            },
            410 => HttpStatusResponse {
                code: 410,
                description: "Gone",
                headers: None,
                exclude_body: false,
                body: None,
            },
            411 => HttpStatusResponse {
                code: 411,
                description: "Length Required",
                headers: None,
                exclude_body: false,
                body: None,
            },
            412 => HttpStatusResponse {
                code: 412,
                description: "Precondition Failed",
                headers: None,
                exclude_body: false,
                body: None,
            },
            413 => HttpStatusResponse {
                code: 413,
                description: "Request Entity Too Large",
                headers: None,
                exclude_body: false,
                body: None,
            },
            414 => HttpStatusResponse {
                code: 414,
                description: "Request-URI Too Long",
                headers: None,
                exclude_body: false,
                body: None,
            },
            415 => HttpStatusResponse {
                code: 415,
                description: "Unsupported Media Type",
                headers: None,
                exclude_body: false,
                body: None,
            },
            416 => HttpStatusResponse {
                code: 416,
                description: "Requested Range Not Satisfiable",
                headers: None,
                exclude_body: false,
                body: None,
            },
            417 => HttpStatusResponse {
                code: 417,
                description: "Expectation Failed",
                headers: None,
                exclude_body: false,
                body: None,
            },
            418 => HttpStatusResponse {
                code: 418,
                description: "I'm a teapot",
                headers: None,
                exclude_body: false,
                body: None,
            },
            421 => HttpStatusResponse {
                code: 421,
                description: "Misdirected Reques",
                headers: None,
                exclude_body: false,
                body: None,
            },
            422 => HttpStatusResponse {
                code: 422,
                description: "Unprocessable Entity",
                headers: None,
                exclude_body: false,
                body: None,
            },
            423 => HttpStatusResponse {
                code: 423,
                description: "Locked",
                headers: None,
                exclude_body: false,
                body: None,
            },
            425 => HttpStatusResponse {
                code: 425,
                description: "Too Early",
                headers: None,
                exclude_body: false,
                body: None,
            },
            426 => HttpStatusResponse {
                code: 426,
                description: "Upgrade Required",
                headers: None,
                exclude_body: false,
                body: None,
            },
            428 => HttpStatusResponse {
                code: 428,
                description: "Precondition Required",
                headers: None,
                exclude_body: false,
                body: None,
            },
            429 => {
                let mut headers = HeaderMap::new();
                headers.insert("Retry-After", "5".parse().unwrap());
                HttpStatusResponse {
                    code: 429,
                    description: "Too Many Requests",
                    headers: Some(headers),
                    exclude_body: false,
                    body: None,
                }
            }
            431 => HttpStatusResponse {
                code: 431,
                description: "Request Header Fields Too Large",
                headers: None,
                exclude_body: false,
                body: None,
            },
            451 => HttpStatusResponse {
                code: 451,
                description: "Unavailable For Legal Reasons",
                headers: None,
                exclude_body: false,
                body: None,
            },
            500 => HttpStatusResponse {
                code: 500,
                description: "Internal Server Error",
                headers: None,
                exclude_body: false,
                body: None,
            },
            501 => HttpStatusResponse {
                code: 501,
                description: "Not Implemented",
                headers: None,
                exclude_body: false,
                body: None,
            },
            502 => HttpStatusResponse {
                code: 502,
                description: "Bad Gateway",
                headers: None,
                exclude_body: false,
                body: None,
            },
            503 => HttpStatusResponse {
                code: 503,
                description: "Service Unavailable",
                headers: None,
                exclude_body: false,
                body: None,
            },
            504 => HttpStatusResponse {
                code: 504,
                description: "Gateway Timeout",
                headers: None,
                exclude_body: false,
                body: None,
            },
            505 => HttpStatusResponse {
                code: 505,
                description: "HTTP Version Not Supported",
                headers: None,
                exclude_body: false,
                body: None,
            },
            506 => HttpStatusResponse {
                code: 506,
                description: "Variant Also Negotiates",
                headers: None,
                exclude_body: false,
                body: None,
            },
            507 => HttpStatusResponse {
                code: 507,
                description: "Insufficient Storage",
                headers: None,
                exclude_body: false,
                body: None,
            },
            508 => HttpStatusResponse {
                code: 508,
                description: "Loop Detecte",
                headers: None,
                exclude_body: false,
                body: None,
            },
            510 => HttpStatusResponse {
                code: 510,
                description: "Not Extende",
                headers: None,
                exclude_body: false,
                body: None,
            },
            511 => HttpStatusResponse {
                code: 511,
                description: "Network Authentication Required",
                headers: None,
                exclude_body: false,
                body: None,
            },
            520 => HttpStatusResponse {
                code: 520,
                description: "Web Server Returned an Unknown Error",
                headers: None,
                exclude_body: false,
                body: None,
            },
            521 => HttpStatusResponse {
                code: 521,
                description: "Web Server Is Down",
                headers: None,
                exclude_body: false,
                body: None,
            },
            522 => HttpStatusResponse {
                code: 522,
                description: "Connection Timed out",
                headers: None,
                exclude_body: false,
                body: None,
            },
            523 => HttpStatusResponse {
                code: 523,
                description: "Origin Is Unreachable",
                headers: None,
                exclude_body: false,
                body: None,
            },
            524 => HttpStatusResponse {
                code: 524,
                description: "A Timeout Occurred",
                headers: None,
                exclude_body: false,
                body: None,
            },
            525 => HttpStatusResponse {
                code: 525,
                description: "SSL Handshake Failed",
                headers: None,
                exclude_body: false,
                body: None,
            },
            526 => HttpStatusResponse {
                code: 526,
                description: "Invalid SSL Certificate",
                headers: None,
                exclude_body: false,
                body: None,
            },
            527 => HttpStatusResponse {
                code: 527,
                description: "Railgun Error",
                headers: None,
                exclude_body: false,
                body: None,
            },
            530 => HttpStatusResponse {
                code: 530,
                description: "",
                headers: None,
                exclude_body: false,
                body: None,
            },
            _ => HttpStatusResponse {
                code: 404,
                description: "Status Code Not Found",
                headers: None,
                exclude_body: false,
                body: None,
            },
        }
    }
}
