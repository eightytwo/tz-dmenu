#lang racket/base

(require racket/date)

(require gregor)
(require shell/pipeline)

(define now (now/moment))

(define config-file
  (string-append
    (path->string (find-system-path 'home-dir))
    ".config/tz_dmenu/config.rkt"))

;; Read the time zones to display from the config file
(define tzs
  (call-with-input-file
    config-file
    (λ (in)
      (read in))))

;; Determine the length of the longest location name
(define longest-location-length
 (string-length
   (caar
     (sort tzs > #:key (λ (x) (string-length (car x)))))))

;; Create a list of pairs, comprising time zone names
;; and the local datetime in those time zones.
(define tzs-with-time
  (for/list ([tz tzs])
    (cons (car tz) (adjust-timezone now (cdr tz)))))

;; Sort this list based on the time at each location
;; with the largest offset first.
(define sorted-tzs
  (sort
    tzs-with-time
    string>?
    #:key (λ (x) (~t (cdr x) "y MM dd HH mm"))))

;; Define a function that appends spaces to each location name
;; so that all times (printed after the names) are aligned.
(define (pad-location location)
  (string-append
    location
    (make-string
      (add1 (- longest-location-length (string-length location)))
      #\space)))

;; Create a list of strings, each made up of a time zone name
;; and the local time in that zone.
(define tz-strings
  (for/list ([tz sorted-tzs])
    (string-append (pad-location (car tz)) (~t (cdr tz) "hh:mm a"))))

;; Run dmenu in a subprocess with its input port as a pipe
(define pipeline
  (run-subprocess-pipeline
   '(dmenu "-i" "-l" "4" "-p" ">")
   #:in #f
   #:background? #t))

;; Write each time zone string to dmenu
(for ([s tz-strings])
  (displayln s (pipeline-port-to pipeline)))
