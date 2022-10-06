# httpavonz

A simple service for generating different HTTP codes. It's a Rust re-implementation of https://httpstat.us, built just for fun.

It's useful for testing how your own scripts deal with varying responses.

## How it works

Just append the status code you want in the URL, like [htt.pavonz.com/200](/200) and it'll return a response like the following:

    HTTP/1.1 <code> <description>
    Content-Type: <text/plain or application/json>
    <other response headers>
    <code> {description}

## Supported status codes

- [100](/100) - _Continue_
- [101](/101) - _Switching Protocols_
- [102](/102) - _Processing_
- [103](/103) - _Early Hints_
- [200](/200) - _OK_
- [201](/201) - _Created_
- [202](/202) - _Accepted_
- [203](/203) - _Non-Authoritative Information_
- [204](/204) - _No Content_
- [205](/205) - _Reset Content_
- [206](/206) - _Partial Content_
- [207](/207)
- [208](/208) - _Already Reported_
- [226](/226)
- [300](/300) - _Multiple Choices_
- [301](/301) - _Moved Permanently_
- [302](/302) - _Found_
- [303](/303) - _See Other_
- [304](/304) - _Not Modified_
- [305](/305) - _Use Proxy_
- [306](/306) - _Switch Proxy_
- [307](/307) - _Temporary Redirect_
- [308](/308) - _Permanent Redirect_
- [400](/400) - _Bad Request_
- [401](/401) - _Unauthorized_
- [402](/402) - _Payment Required_
- [403](/403) - _Forbidden_
- [404](/404) - _Not Found_
- [405](/405) - _Method Not Allowed_
- [406](/406) - _Not Acceptable_
- [407](/407) - _Proxy Authentication Required_
- [408](/408) - _Request Timeout_
- [409](/409) - _Conflict_
- [410](/410) - _Gone_
- [411](/411) - _Length Required_
- [412](/412) - _Precondition Failed_
- [413](/413) - _Request Entity Too Large_
- [414](/414) - _Request-URI Too Long_
- [415](/415) - _Unsupported Media Type_
- [416](/416) - _Requested Range Not Satisfiable_
- [417](/417) - _Expectation Failed_
- [418](/418)
- [421](/421) - _Misdirected Request_
- [422](/422) - _Unprocessable Entity_
- [423](/423) - _Locked_
- [425](/425) - _Too Early_
- [426](/426) - _Upgrade Required_
- [428](/428) - _Precondition Required_
- [429](/429) - _Too Many Requests_
- [431](/431) - _Request Header Fields Too Large_
- [451](/451) - _Unavailable For Legal Reasons_
- [500](/500) - _Internal Server Error_
- [501](/501) - _Not Implemented_
- [502](/502) - _Bad Gateway_
- [503](/503) - _Service Unavailable_
- [504](/504) - _Gateway Timeout_
- [505](/505) - _HTTP Version Not Supported_
- [506](/506) - _Variant Also Negotiates_
- [507](/507) - _Insufficient Storage_
- [508](/508)
- [510](/510)
- [511](/511) - _Network Authentication Required_
- [520](/520) - _Web Server Returned an Unknown Error_
- [521](/521) - _Web Server Is Down_
- [522](/522) - _Connection Timed out_
- [523](/523) - _Origin Is Unreachable_
- [524](/524) - _A Timeout Occurred_
- [525](/525) - _SSL Handshake Failed_
- [526](/526) - _Invalid SSL Certificate_
- [527](/527) - _Railgun Error_
- [530](/530) -

## Other features

#### JSON responses

To get a JSON response back, you need to set `Accept` header to `application/json`. Then it'll JSON encode the response and send the `Content-Type` header accordingly.

#### Delayed replies

If you want a delay on the response add a query string `sleep` (the time is in ms, max 30 seconds), like this: [htt.pavonz.com/200?sleep=5000](/200?sleep=5000)

#### CORS

All endpoints have been configured to allow all origins, headers, and HTTP methods.

#### Unknown codes

If you send any other three digit number that's not in that list, it'll return it too. Or, [send a PR](https://github.com/andreapavoni/httpavonz) to add full support for a new code.

---

©2022 a [pavonz](https://pavonz.com) joint - [src](https://github.com/andreapavoni/httpavonz) - _No data about the requests you make is captured or stored._
