// Discovery-Provider: WebradioDB
// Description: Community-kuratierte Radios, Alternative zu Radio-Browser.info
// Hinweis: Diese DB enthält nur Metadaten, keine Stream-URLs.
// Die Stationen dienen als Referenz (Name, Tags, Homepage).

let base_url = "https://jcorporation.github.io/webradiodb/";
let url = base_url + "db/index/webradios.min.json";

let raw = #{};
try {
   raw = http_get_json_relaxed(url);
} catch (error) {
   // API nicht erreichbar, leeres Ergebnis zurückgeben
};

let results = [];
let station_keys = raw.keys();
for key in station_keys {
   let station = raw[key];
   let name = if station.Name == () { key } else { station.Name };
   let genre_arr = station.Genre;
   let tags = "";
   if genre_arr != () {
      let parts = [];
      for g in genre_arr {
         parts.push(g);
      }
      if parts.len() > 0 {
         tags = parts[0];
         let i = 1;
         while i < parts.len() {
            tags += "," + parts[i];
            i += 1;
         }
      }
   }
   let img = if station.Image == () { "" } else { base_url + station.Image };
   let hp = if station.Homepage == () { "" } else { station.Homepage };

   results.push(#{
      "name": name,
      "stream_url": "",
      "stream_urls": [],
      "homepage": hp,
      "icon_url": img,
      "tags": tags,
      "country": "",
      "language": ""
   });
}

json_stringify(#{
   "provider": "WebradioDB",
   "stations": results
})
