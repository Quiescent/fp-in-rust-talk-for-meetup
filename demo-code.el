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
  (progn
    (demo-it-presentation-advance)
    (fir-load-line-in-main 5 3)))

(defun fir-load-imperative-solution ()
  "Load the series of approximations snippet of code."
  (progn
    (fir-close-code-window)
    (demo-it-presentation-advance)
    (fir-load-line-in-main 9 8)))

;; (defun  ()
;;   "Load the imperative approximations code."
;;   (progn
;;     (fir-close-code-window)
;;     (demo-it-presentation-advance)
;;     (fir-load-line-in-main )))

(defconst fir-main-buffer-name "*main*"
  "The name given to the buffer in which we insert the contents of main.")

(defconst fir-cargo-run-shell-name "*Cargo-Run*"
  "The name given to the buffer for running cargo.")

(defun fir-run-imperative-solution ()
  "Run the imperative solution to square roots."
  (progn
    (fir-close-code-window)
    (demo-it-presentation-advance)
    (demo-it--make-side-window :right)
    (switch-to-buffer fir-main-buffer-name)
    (rust-mode)
    (text-scale-set 1)
    (demo-it-insert "fn main() {\n    println!(\"sqrt 5: {:?}\", naive_sqrts(5.0, 10));\n}" :fast)
    (demo-it-start-shell (concat default-directory "re-implement-std-lib/newton-rhapson-square-roots/")
                         nil
                         fir-cargo-run-shell-name)
    (demo-it-insert "cargo run" :medium)
    (eshell-send-input)))

(defun fir-cleanup ()
  "Cleanup all temp buffers used in presentation."
  (progn
    (ignore-errors (kill-buffer (concat "Shell: " fir-cargo-run-shell-name)))
    (ignore-errors (kill-buffer fir-main-buffer-name))
    (thread-last (buffer-list)
      (cl-remove-if-not (lambda (buffer) (string-match "main\\.rs"
                                                       (buffer-name buffer))))
      (mapc #'kill-buffer))))

(defun fir-close-code-window ()
  "Close the code window."
  (progn
    (other-window 1)
    (delete-other-windows)
    (fir-cleanup)))

(defun fir-write-iterator-example ()
  "Write some code to show how awesome iterators are!"
  (progn
    (fir-close-code-window)
    (demo-it-presentation-advance)
    (demo-it--make-side-window :right 100)
    (switch-to-buffer fir-main-buffer-name)
    (rust-mode)
    (text-scale-set 1)
    (demo-it-insert "pub fn from_signal(signal: &Signal) -> Correlation {\n        let samples = &signal.samples;\n        Correlation {\n            value: (0..samples.len()).map(|offset| {\n                samples.iter().take(samples.len() - offset)\n                    .zip(samples.iter().skip(offset))\n                    .map(|(sig_i, sig_j)| sig_i * sig_j)\n                    .sum()\n            }).collect()\n        }\n    }" :fast)))

(defun fir-load-iterator-definition ()
  "Load my implementation of iterators."
  (progn
    (fir-close-code-window)
    (demo-it-presentation-advance)
    (demo-it-load-file "re-implement-std-lib/newton-rhapson-square-roots/src/iterable/iterable.rs"
                       :side
                       1
                       100)))

(defun fir-setup-presentation ()
  "Entry point for the code presentation."
  (interactive)
  (progn
    (fir-cleanup)
    (demo-it-create :single-window
                    (demo-it-presentation "Functional Programming in Rust.org")
                    fir-load-next-approximation
                    fir-load-imperative-solution
                    fir-run-imperative-solution
                    fir-load-imperative-solution
                    fir-write-iterator-example
                    fir-load-iterator-definition)
    (demo-it-start)))

(provide 'demo-code)
;;; demo-code ends here
