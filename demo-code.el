;;; demo-code --- a demo-it presentation for the code part of my talk -*- lexical-binding: t; -*-

;;; Commentary:

;;; Code:

(require 'demo-it)

(defun fir-load-line-in-main (start count)
  "Present a section of the main file.

Show code from line START, ending COUNT lines after that."
  (progn
    (demo-it-presentation-advance)
    (demo-it-load-fancy-file "newton-rhapson-square-roots/src/main.rs" :line start (+ start count) :side)))

(defun fir-load-next-approximation ()
  "Load the next approximation snippet of code."
  (fir-load-line-in-main 5 3))

(defun fir-load-series-of-approximations ()
  "Load the series of approximations snippet of code."
  (fir-load-line-in-main 9 8))

(defun fir-setup-presentation ()
  "Entry point for the code presentation."
  (interactive)
  (progn
    (demo-it-create :single-window
                    (demo-it-title-screen "code-title.org")
                    (demo-it-presentation "code-presentation.org")
                    fir-load-next-approximation
                    fir-load-series-of-approximations)
    (demo-it-start)))

(provide 'demo-code)
;;; demo-code ends here
