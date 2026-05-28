// Discovery-Provider: WebradioDB
// Description: Community-kuratierte Radios, Alternative zu Radio-Browser.info

let url = "https://jcorporation.github.io/webradiodb/db/index/webradios.min.json";

let raw = #{};
try {
   raw = http_get_json(url);
} catch (error) {
   // API nicht erreichbar, leeres Ergebnis zurückgeben
};

let results = [];
let station_keys = raw.keys();
for key in station_keys {
   let station = raw[key];
   let stream_url = if station.url == () { "" } else { station.url };
   let urls = [];
   if stream_url != "" {
      urls.push(#{
         "url": station.url,
         "codec": station.codec,
         "bitrate": station.bitrate,
         "tls": false
      });
   }

   results.push(#{
      "name": station.name,
      "stream_url": stream_url,
      "stream_urls": urls,
      "homepage": station.homepage,
      "icon_url": station.favicon,
      "tags": station.tags,
      "country": station.country,
      "language": station.language
   });
}

json_stringify(#{
   "provider": "WebradioDB",
   "stations": results
})
