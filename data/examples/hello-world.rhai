// Discovery-Provider: Hello World
// Description: Simplest possible discovery script — returns one station

json_stringify(#{
    "provider": "Hello World",
    "stations": [
        #{
            "name": "Public Domain Jazz",
            "stream_url": "https://example.com/jazz.ogg",
            "stream_urls": [
                #{ "url": "https://example.com/jazz.ogg", "codec": "OGG", "bitrate": 192, "tls": true }
            ],
            "homepage": "https://example.com",
            "icon_url": "https://example.com/icon.png",
            "tags": "jazz,public domain",
            "country": "US",
            "language": "en"
        }
    ]
})
