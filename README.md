# httpavonz

A simple service for generating different HTTP codes. It's a Rust re-implementation of https://httpstat.us, built just for fun.

It's useful for testing how your http clients deal with varying responses.

## How it works

Just append the status code you want in the URL, like [https://htt.pavonz.com/200](/200) and it'll return a response like the following:

```sh
$ curl -v "https://htt.pavonz.com/401"
*   Trying 1.2.3.4:80...
* Connected to htt.pavonz.com (1.2.3.4) port 80 (#0)
> GET /401 HTTP/1.1
> Host: htt.pavonz.com
> User-Agent: curl/7.79.1
> Accept: */*
>
* Mark bundle as not supporting multiuse
< HTTP/1.1 401 Unauthorized
< content-type: text/plain; charset=utf-8
< www-authenticate: Basic realm="Fake Realm"
< content-length: 16
< server: httpavonz
< access-control-allow-origin: *
< vary: origin
< vary: access-control-request-method
< vary: access-control-request-headers
< access-control-expose-headers: link,content-range,location,www-authenticate,proxy-authenticate,retry-after
< date: Fri, 07 Oct 2022 09:48:38 GMT
<
* Connection #0 to host htt.pavonz.com left intact
401 Unauthorized
```

## Supported status codes

- [100](https://htt.pavonz.com/100) - _Continue_
- [101](https://htt.pavonz.com/101) - _Switching Protocols_
- [102](https://htt.pavonz.com/102) - _Processing_
- [103](https://htt.pavonz.com/103) - _Early Hints_
- [200](https://htt.pavonz.com/200) - _OK_
- [201](https://htt.pavonz.com/201) - _Created_
- [202](https://htt.pavonz.com/202) - _Accepted_
- [203](https://htt.pavonz.com/203) - _Non-Authoritative Information_
- [204](https://htt.pavonz.com/204) - _No Content_
- [205](https://htt.pavonz.com/205) - _Reset Content_
- [206](https://htt.pavonz.com/206) - _Partial Content_
- [207](https://htt.pavonz.com/207)
- [208](https://htt.pavonz.com/208) - _Already Reported_
- [226](https://htt.pavonz.com/226)
- [300](https://htt.pavonz.com/300) - _Multiple Choices_
- [301](https://htt.pavonz.com/301) - _Moved Permanently_
- [302](https://htt.pavonz.com/302) - _Found_
- [303](https://htt.pavonz.com/303) - _See Other_
- [304](https://htt.pavonz.com/304) - _Not Modified_
- [305](https://htt.pavonz.com/305) - _Use Proxy_
- [306](https://htt.pavonz.com/306) - _Switch Proxy_
- [307](https://htt.pavonz.com/307) - _Temporary Redirect_
- [308](https://htt.pavonz.com/308) - _Permanent Redirect_
- [400](https://htt.pavonz.com/400) - _Bad Request_
- [401](https://htt.pavonz.com/401) - _Unauthorized_
- [402](https://htt.pavonz.com/402) - _Payment Required_
- [403](https://htt.pavonz.com/403) - _Forbidden_
- [404](https://htt.pavonz.com/404) - _Not Found_
- [405](https://htt.pavonz.com/405) - _Method Not Allowed_
- [406](https://htt.pavonz.com/406) - _Not Acceptable_
- [407](https://htt.pavonz.com/407) - _Proxy Authentication Required_
- [408](https://htt.pavonz.com/408) - _Request Timeout_
- [409](https://htt.pavonz.com/409) - _Conflict_
- [410](https://htt.pavonz.com/410) - _Gone_
- [411](https://htt.pavonz.com/411) - _Length Required_
- [412](https://htt.pavonz.com/412) - _Precondition Failed_
- [413](https://htt.pavonz.com/413) - _Request Entity Too Large_
- [414](https://htt.pavonz.com/414) - _Request-URI Too Long_
- [415](https://htt.pavonz.com/415) - _Unsupported Media Type_
- [416](https://htt.pavonz.com/416) - _Requested Range Not Satisfiable_
- [417](https://htt.pavonz.com/417) - _Expectation Failed_
- [418](https://htt.pavonz.com/418)
- [421](https://htt.pavonz.com/421) - _Misdirected Request_
- [422](https://htt.pavonz.com/422) - _Unprocessable Entity_
- [423](https://htt.pavonz.com/423) - _Locked_
- [425](https://htt.pavonz.com/425) - _Too Early_
- [426](https://htt.pavonz.com/426) - _Upgrade Required_
- [428](https://htt.pavonz.com/428) - _Precondition Required_
- [429](https://htt.pavonz.com/429) - _Too Many Requests_
- [431](https://htt.pavonz.com/431) - _Request Header Fields Too Large_
- [451](https://htt.pavonz.com/451) - _Unavailable For Legal Reasons_
- [500](https://htt.pavonz.com/500) - _Internal Server Error_
- [501](https://htt.pavonz.com/501) - _Not Implemented_
- [502](https://htt.pavonz.com/502) - _Bad Gateway_
- [503](https://htt.pavonz.com/503) - _Service Unavailable_
- [504](https://htt.pavonz.com/504) - _Gateway Timeout_
- [505](https://htt.pavonz.com/505) - _HTTP Version Not Supported_
- [506](https://htt.pavonz.com/506) - _Variant Also Negotiates_
- [507](https://htt.pavonz.com/507) - _Insufficient Storage_
- [508](https://htt.pavonz.com/508)
- [510](https://htt.pavonz.com/510)
- [511](https://htt.pavonz.com/511) - _Network Authentication Required_
- [520](https://htt.pavonz.com/520) - _Web Server Returned an Unknown Error_
- [521](https://htt.pavonz.com/521) - _Web Server Is Down_
- [522](https://htt.pavonz.com/522) - _Connection Timed out_
- [523](https://htt.pavonz.com/523) - _Origin Is Unreachable_
- [524](https://htt.pavonz.com/524) - _A Timeout Occurred_
- [525](https://htt.pavonz.com/525) - _SSL Handshake Failed_
- [526](https://htt.pavonz.com/526) - _Invalid SSL Certificate_
- [527](https://htt.pavonz.com/527) - _Railgun Error_
- [530](https://htt.pavonz.com/530)

## Other features

#### JSON responses

To get a JSON response back, you need to set `Accept` header to `application/json`. Then it'll JSON encode the response and send the `Content-Type` header accordingly.

#### Delayed replies

If you want a delay on the response add a query string `sleep` (the time is in ms, max 30 seconds), for example: [https://htt.pavonz.com/200?sleep=5000](/200?sleep=5000)

#### CORS

All endpoints have been configured to allow all origins, headers, and HTTP methods.

#### Unknown codes

If you send any other three digit number that's not in that list, it'll return it too. Or, [send a PR](https://github.com/andreapavoni/httpavonz) to add full support for a new code.

## Up and running

#### Locally

Using `Cargo`:

```sh
$ cargo run
```

or Docker:

```sh
$ docker run --rm --env-file .env.dist -it -p 8080:8080 $(docker build .)
```

then try a request:

```sh
$ curl -v "https://127.0.0.1:8080/401"
```

#### Deploy

Depending on the provider, there isn't a standard procedure.

## Credits

The authors of [httpstat.us](https://httpstat.us) for their brillant idea and execution.

---

©2022 a [pavonz](https://pavonz.com) joint
