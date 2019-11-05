;;; demo-code --- a demo-it presentation for the code part of my talk -*- lexical-binding: t; -*-

;;; Commentary:

;;; Code:

(require 'demo-it)
(require 'subr-x)

(defun fir-load-line-in-main (start count)
  "Present a section of the main file.

Show code from line START, ending COUNT lines after that."
  (progn
    (demo-it-load-part-file "re-implement-std-lib/newton-rhapson-square-roots/src/main.rs"
                            :line start (+ start count)
                            :side
                            1
                            100)))

(defun fir-load-next-approximation ()
  "Load the next approximation snippet of code."
  (demo-it-presentation-advance)
  (fir-load-line-in-main 5 3))

(defun fir-load-series-of-approximations ()
  "Load the series of approximations snippet of code."
  (progn
    (fir-close-code-window)
    (demo-it-presentation-advance)
    (fir-load-line-in-main 9 8)))

(defun fir-cleanup ()
  "Cleanup all temp buffers used in presentation."
  (thread-last (buffer-list)
    (cl-remove-if-not (lambda (buffer) (string-match "main\\.rs"
                                                     (buffer-name buffer))))
    (mapc #'kill-buffer)))

(defun fir-close-code-window ()
  "Close the code window."
  (progn
    (other-window 1)
    (delete-other-windows)
    (fir-cleanup)))

(defun fir-setup-presentation ()
  "Entry point for the code presentation."
  (interactive)
  (progn
    (fir-cleanup)
    (demo-it-create :single-window
                    (demo-it-presentation "Functional Programming in Rust.org")
                    fir-load-next-approximation
                    fir-load-series-of-approximations)
    (demo-it-start)))

(provide 'demo-code)
;;; demo-code ends here
