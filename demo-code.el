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
    (ignore-errors (kill-buffer "iterable.rs"))
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

(defun fir-narrow-to-trait ()
  "Narrow to the part defining trait in my implementation of iterators."
  (progn
    (widen)
    (narrow-to-region 1 105)))

(defun fir-narrow-to-take ()
  "Narrow to the implementation of take in my implementation of iterators."
  (progn
    (widen)
    (narrow-to-region 106 341)))

(defun fir-narrow-to-drop ()
  "Narrow to the implementation of drop in my implementation of iterators."
  (progn
    (widen)
    (narrow-to-region 342 455)))

(defun fir-narrow-to-not-working-function ()
  "Narrow to the implementation of drop_while in my implementation of iterators."
  (progn
    (widen)
    (narrow-to-region 456 710)))

(defun fir-load-square-root-struct ()
  "Load the square root struct code."
  (progn
    (fir-close-code-window)
    (demo-it-presentation-advance)
    (fir-load-line-in-main 27 4)))

(defun fir-load-iterator-impl ()
  "Load the implementation of square root."
  (progn
    (fir-close-code-window)
    (fir-load-line-in-main 32 20)))

(defun fir-load-heuristics ()
  "Load the iteration code."
  (progn
    (fir-close-code-window)
    (fir-load-line-in-main 53 15)))

(defun fir-load-main-and-run ()
  "Load main and run it."
  (progn
    (fir-close-code-window)
    (fir-load-line-in-main 69 27)
    (other-window 1)
    (demo-it-start-shell (concat default-directory "re-implement-std-lib/newton-rhapson-square-roots/")
                         nil
                         fir-cargo-run-shell-name)
    (demo-it-insert "cargo run --release" :medium)
    (eshell-send-input)))

(defun fir-load-main-and-run-all ()
  "Load main and run it."
  (progn
    (fir-close-code-window)
    (fir-load-line-in-main 69 27)
    (uncomment-region 2232 2703)
    (save-buffer)
    (other-window 1)
    (demo-it-start-shell (concat default-directory "re-implement-std-lib/newton-rhapson-square-roots/")
                         nil
                         fir-cargo-run-shell-name)
    (demo-it-insert "cargo run --release" :medium)
    (eshell-send-input)))

(defun fir-load-line-in-iter-main (start count)
  "Present a section of the main file.

Show code from line START, ending COUNT lines after that."
  (progn
    (demo-it-load-part-file "using-iter/newton-rhapson-square-roots/src/main.rs"
                            :line start (+ start count)
                            :side
                            1
                            100)))

(defun fir-load-successor-based-approximation ()
  "Load my implementation in terms of Successor."
  (progn
    (fir-close-code-window)
    (demo-it-presentation-advance)
    (fir-load-line-in-iter-main 46 3)))

(defun fir-load-successor-based-heuuristics ()
  "Load my implementation of the heuristics in terms of Successor."
  (progn
    (fir-close-code-window)
    (demo-it-presentation-advance)
    (fir-load-line-in-iter-main 23 19)))

(defun fir-load-successor-based-main ()
  "Load my main implementation of newton-rhapson in terms of Successor."
  (progn
    (fir-close-code-window)
    (demo-it-presentation-advance)
    (fir-load-line-in-iter-main 50 11)))

(defun fir-load-non-working-code ()
  "Load my main implementation of newton-rhapson in terms of Successor."
  (progn
    (fir-close-code-window)
    (demo-it-presentation-advance)
    (fir-load-line-in-iter-main 15 7)))

(defun fir-load-line-in-diff-main (start count)
  "Present a section of the main file.

Show code from line START, ending COUNT lines after that."
  (progn
    (demo-it-load-part-file "using-iter/numerical-differentiation/src/main.rs"
                            :line start (+ start count)
                            :side
                            1
                            100)))

(defun fir-load-slope-code ()
  "Load the code which computes slope."
  (progn
    (fir-close-code-window)
    (demo-it-presentation-advance)
    (fir-load-line-in-diff-main 23 3)))

(defun fir-load-differentiate-approximations-code ()
  "Load the code which computes slope."
  (progn
    (fir-close-code-window)
    (demo-it-presentation-advance)
    (fir-load-line-in-diff-main 30 6)))

(defun fir-load-main-diff-within-code ()
  "Load the code which computes within and relative for diff."
  (progn
    (fir-close-code-window)
    (demo-it-presentation-advance)
    (fir-load-line-in-diff-main 63 11)))

(defun fir-load-eliminate-error-code ()
  "Load the code which eliminates errors for diff."
  (progn
    (fir-close-code-window)
    (demo-it-presentation-advance)
    (fir-load-line-in-diff-main 37 25)))

(defun fir-load-main-eliminate-errors ()
  "Load the main code which eliminates errors for diff."
  (progn
    (fir-close-code-window)
    (demo-it-presentation-advance)
    (fir-load-line-in-diff-main 74 17)))

(defun fir-run-diff-code ()
  "Run the main file which I wrote for differentiating."
  (progn
    (other-window 1)
    (demo-it-start-shell (concat default-directory "re-implement-std-lib/newton-rhapson-square-roots/")
                         nil
                         fir-cargo-run-shell-name)
    (demo-it-insert "cargo run --release" :medium)
    (eshell-send-input)))

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
                    fir-load-iterator-definition
                    fir-narrow-to-trait
                    fir-narrow-to-take
                    fir-narrow-to-drop
                    fir-narrow-to-not-working-function
                    fir-load-square-root-struct
                    fir-load-iterator-impl
                    fir-load-heuristics
                    fir-load-main-and-run
                    fir-load-main-and-run-all
                    fir-load-successor-based-approximation
                    fir-load-successor-based-heuuristics
                    fir-load-successor-based-main
                    fir-load-non-working-code
                    fir-load-slope-code
                    fir-load-differentiate-approximations-code
                    fir-load-main-diff-within-code
                    fir-load-eliminate-error-code
                    fir-load-main-eliminate-errors
                    fir-run-diff-code)
    (demo-it-start)))

(provide 'demo-code)
;;; demo-code ends here
