// Discovery-Provider: SomaFM
// Description: 30+ kuratierte Channels (ambient, electronic, indie, ...)

let html = http_get("https://somafm.com/");
let links_json = html_select(html, "a[href^='/player24/station/']");
let links = json_parse(links_json);

let seen = #{};
let stations = [];

// SomaFM-Naming: {slug}.pls = MP3, {slug}128.pls = MP3@128, {slug}130.pls = AAC@128, etc.
let formats = [
    #{ "suffix": ".pls",   "codec": "MP3", "bitrate": 256, "tls": true,  "subdir": ""       },
    #{ "suffix": "128.pls","codec": "MP3", "bitrate": 128, "tls": true,  "subdir": ""       },
    #{ "suffix": "130.pls","codec": "AAC", "bitrate": 128, "tls": true,  "subdir": ""       },
    #{ "suffix": "64.pls", "codec": "AAC", "bitrate": 64,  "tls": true,  "subdir": ""       },
    #{ "suffix": "32.pls", "codec": "AAC", "bitrate": 32,  "tls": true,  "subdir": ""       },
    #{ "suffix": ".pls",   "codec": "MP3", "bitrate": 256, "tls": false, "subdir": "/nossl" },
    #{ "suffix": "128.pls","codec": "MP3", "bitrate": 128, "tls": false, "subdir": "/nossl" },
    #{ "suffix": "130.pls","codec": "AAC", "bitrate": 128, "tls": false, "subdir": "/nossl" },
    #{ "suffix": "64.pls", "codec": "AAC", "bitrate": 64,  "tls": false, "subdir": "/nossl" },
    #{ "suffix": "32.pls", "codec": "AAC", "bitrate": 32,  "tls": false, "subdir": "/nossl" },
];

for link in links {
    let href = link.attrs["href"];
    let slug = href.split("/")[3];

    if seen.contains(slug) { continue; }
    seen[slug] = true;

    let children_json = html_select(link.html, "img");
    let children = json_parse(children_json);
    let name = "";
    let icon_url = "";
    if children.len() > 0 {
        let alt = children[0].attrs["alt"];
        icon_url = "https://somafm.com" + children[0].attrs["src"];
        name = alt.split(":")[0];
    }

    let stream_urls = [];
    let primary_url = "";
    for f in formats {
        let url = "https://somafm.com" + f.subdir + "/" + slug + f.suffix;
        stream_urls.push(#{
            "url": url,
            "codec": f.codec,
            "bitrate": f.bitrate,
            "tls": f.tls
        });
        if primary_url == "" { primary_url = url; }
    }

    stations.push(#{
        "name": name,
        "stream_url": primary_url,
        "stream_urls": stream_urls,
        "homepage": "https://somafm.com/" + slug + "/",
        "icon_url": icon_url,
        "tags": "soma fm,electronic,independent",
        "country": "US",
        "language": "en"
    });
}

json_stringify(#{
    "provider": "SomaFM",
    "stations": stations
})
