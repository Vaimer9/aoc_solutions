#lang racket

(define input (file->lines "input1.txt"))

;; Part 1
;; filter out all numbers and put the first and last into a string
(define (find_calibration_p1 s)
  (let ([n (filter char-numeric? (string->list s))])
    (string->number (list->string (list (first n) (last n))))))

(define p1_ans 
 (apply + (map find_calibration_p1 input)))

;; Part 2

(define lookup (hash
    "one" "1"
    "two" "2"
    "three" "3"
    "four" "4"
    "five" "5"
    "six" "6"
    "seven" "7"
    "eight" "8"
    "nine" "9"
    "1" "1"
    "2" "2"
    "3" "3"
    "4" "4"
    "5" "5"
    "6" "6"
    "7" "7"
    "8" "8"
    "9" "9"
  ))

;; Look up one by one if elements of lookup are in input string, if yes, store them in a list
;; Im new to racket so.. Thanks to u/anyusername12
(define (val-in-list s)
  (for*/list ([i (in-range 0 (string-length s))]
              [(key val) (in-hash lookup)]
    #:when (and (<= (+ i (string-length key)) (string-length s))
                (equal? key (substring s i (+ i (string-length key))))))
    val))

(define (fnl s) ;; First and Last
  (let ([xs (string->list s)])
    (list->string (list (first xs) (last xs)))))

(define p2_ans
  (apply + (map (compose string->number (λ (s) (apply (compose fnl string-append) (val-in-list s))))
    input)))

(printf "Part 1: ~v\nPart 2: ~v\n" p1_ans p2_ans)
