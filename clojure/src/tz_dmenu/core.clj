(ns tz-dmenu.core
  (:require [clojure.edn :as edn]
            [clojure.string :as str]
            [me.raynes.conch :as sh]
            [tick.alpha.api :as t])
  (:gen-class))

(defn get-config-path
  "Get the full path to the tz_dmenu config file."
  []
  (let [separator (System/getProperty "file.separator")
        home (System/getProperty "user.home")
        segments [home ".config" "tz_dmenu" "config.edn"]]
    (str/join separator segments)))

(defn read-config
  "Read the contents of the tz_dmenu config file."
  []
  (let [path (get-config-path)]
    (try
      (edn/read-string (slurp path))
      (catch java.io.FileNotFoundException _
        (prn
         (str "Unable to read the config file, "
              "please ensure it exists at" path))
        (System/exit 1)))))

(defn calculate-current-time
  "Get the current time in the time zone described by `tz-map`.
  Return a new map describing this time zone, including the
  current time as a zoned date time, this time formatted for
  display, and the length of the title to be displayed."
  [tz-map]
  (let [{tz :tz name :name} tz-map
        now (t/in (t/zoned-date-time) tz)
        formatted-now (t/format "h:mm a Z" now)]
    (assoc tz-map
           :time now
           :formatted-time formatted-now
           :title-length (+ (count name) (count formatted-now)))))

(defn build-title
  "Build the title to be displayed for a time zone described by
  `tz-map`. The space between the time zone name and the time
  is padded until `max-length` is reached."
  [tz-map max-length]
  (let [{name :name time :formatted-time} tz-map
        pad-length (+ (- max-length (count (str name time))) 2)]
    (str name (apply str (repeat pad-length \space)) time)))

(defn build-titles
  "Build a title for each time zone in the `timezones` collection."
  [timezones]
  (let [max-length (apply max (map :title-length timezones))]
    (map #(build-title % max-length) timezones)))

(defn sort-timezones
  "Sort the time zones in the collection `timezones` in descending order.
  TODO: It should be possible to use `(sort-by after? timezones)` however
  time zones with an offset of zero don't seem to sort correctly."
  [timezones]
  (reverse
   (sort-by #(t/format (t/formatter :iso-offset-date-time) (:time %))
            timezones)))

(defn get-timezones
  "Get the time zones to be displayed."
  []
  (->> (read-config)
       (map calculate-current-time)
       (sort-timezones)
       (build-titles)))

(defn -main
  "Display the current time at each time zone listed in
  ~/.config/tz_dmenu.edn via dmenu."
  []
  (let [timezones (get-timezones)
        num-timezones (count timezones)
        timezones-string (str/join "\n" timezones)]
    (sh/programs dmenu)
    (dmenu "-i" "-l" num-timezones "-p" ">" {:in timezones-string})))
