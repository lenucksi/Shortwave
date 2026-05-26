// Discovery-Provider: WebradioDB
// Description: Community-kuratierte Radios, Alternative zu Radio-Browser.info

let url = "https://api.webradiodb.com/v1/stations";

let result = try http_get_json(url) catch {
   // API nicht erreichbar, leeres Ergebnis zurückgeben
   json_stringify(#{
      "provider": "WebradioDB",
      "stations": []
   })
};

let stations = [];

for station in result {
   let stream_url = station.url;
   let urls = [];
   if station.url != "" {
      urls.push(#{
         "url": station.url,
         "codec": station.codec,
         "bitrate": station.bitrate,
         "tls": false
      });
   }

   stations.push(#{
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
   "stations": stations
})
