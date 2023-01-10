#!/bin/bash

# set up a bare .emacs file to install packages
cat > ~/.emacs <<EOF
(require 'package)
(add-to-list 'package-archives '("melpa" . "http://melpa.org/packages/"))
(package-initialize)
(package-refresh-contents)
EOF



# now list the packages to install
cat > package-list <<EOF
(package-install 'cl-lib)
(package-install 'list-utils)
(package-install 'load-relative)
(package-install 'loc-changes)
(package-install 'company)
(package-install 'dash)
(package-install 'dash-functional)
(package-install 'elquery)
(package-install 'epl)
(package-install 'f)
(package-install 'flycheck)
(package-install 'gitlab-ci-mode)
(package-install 'gitlab-ci-mode-flycheck)
(package-install 'ht)
(package-install 'lsp-mode)
(package-install 'lsp-ui)
(package-install 'markdown-mode)
(package-install 'pcache)
(package-install 'pkg-info)
(package-install 'popup)
(package-install 'rainbow-delimiters)
(package-install 'realgud)
(package-install 'request)
(package-install 'rtags)
(package-install 's)
(package-install 'spinner)
(package-install 'srefactor)
(package-install 'test-simple)
(package-install 'xterm-color)
(package-install 'yaml-mode)
(package-install 'yasnippet)
(package-install 'clang-format)
(package-install 'company-c-headers)
(package-install 'company-rtags)
(package-install 'flycheck-gradle)
(package-install 'flycheck-popup-tip)
(package-install 'gradle-mode)
(package-install 'java-imports)
(package-install 'javadoc-lookup)
(package-install 'jtags)
(package-install 'lsp-java)
(package-install 'lsp-javacomp)
(package-install 'memoize)
(package-install 'magit)
(package-install 'forge)
(package-install 'dap-mode)
(package-install 'dracula-theme)
(package-install 'groovy-mode)
(package-install 'helm-lsp)
(package-install 'use-package)
(package-install 'lsp-ivy)
EOF

echo "y" | emacs -nw --batch -u `whoami` --script package-list

rm package-list
mkdir -p ~/.emacs.d/dcoverage

cat > ~/.emacs.d/dcoverage/dcoverage.el <<EOF
;;; package --- Summary
;;;  Display code coverage

;;; Commentary:
;;;   cov.el doesn't work in console mode, and doesn't show partially covered branches.
;;;   It also doesn't show a summary after tests are run

;;; Code:
(require 'elquery)
(require 'gradle-mode)

(defgroup dcoverage nil "Shows unit test coverage in colors in source code buffers.")

(defcustom dcoverage-covered-stm-color  "#008700"
  "The color to use for a statement that got covered by the tests."
  :tag "dcoverage covered statement color"
  :group 'dcoverage
  :type 'color)

(defcustom dcoverage-uncovered-stm-color  "#870000"
  "The color to use for a statement that was not covered by the tests."
  :tag "dcoverage uncovered statement color"
  :group 'dcoverage
  :type 'color)

(defcustom dcoverage-part-covered-branch-color  "#878700"
  "The color to use for a branch that was partially covered."
  :tag "dcoverage part covered branch color"
  :group 'dcoverage
  :type 'color)


(defcustom dcoverage-well-covered-report-color  "green"
  "The color to use to report a 100% covered class."
  :tag "dcoverage well covered report color"
  :group 'dcoverage
  :type 'color)

(defcustom dcoverage-moderate-covered-report-color  "dark orange"
  "The color to use to report a moderately covered class."
  :tag "dcoverage moderately covered report color"
  :group 'dcoverage
  :type 'color)

(defcustom dcoverage-poorly-covered-report-color  "red"
  "The color to use to report a pooly covered class."
  :tag "dcoverage poorly covered report color"
  :group 'dcoverage
  :type 'color)


(defcustom dcoverage-run-clover-function (lambda() (gradle-run "clean cloverGenerateReport"))
  "The elisp function to execute in order to run clover to generate test coverage results."
  :tag "dcoverage clover execution function"
  :group 'dcoverage
  :type 'function)

(defcustom dcoverage-key-generate-and-show (kbd "C-c C-t")
  "The keyboard sequence to generate results and show them."
  :tag "dcoverage key sequence generation and show"
  :group 'dcoverage
  :type 'key-sequence)

(defcustom dcoverage-key-clear-all (kbd "C-c -")
  "The keyboard sequence to clear coloration from all buffs."
  :tag "dcoverage key sequence clear"
  :group 'dcoverage
  :type 'key-sequence)

(defcustom dcoverage-package-name-elide ""
  "Elide this string from the start of package names when displaying class names."
  :tag "dcoverage package name elide"
  :group 'dcoverage
  :type 'string)

(defun dcoverage-find-project-root ()
  "Find the project root, by looking for build.gradle file."
  (let ((dir (locate-dominating-file default-directory "build.gradle")))
    (if dir dir default-directory)))

(defun dcoverage-default-find-cov-file ()
  "Default way to find coverage file: find project root, then go to 'build/reports/clover/clover.xml'."
  (concat (file-name-as-directory (dcoverage-find-project-root)) "build/reports/clover/clover.xml"))

(defvar dcoverage-find-cov-file-fn (symbol-function 'dcoverage-default-find-cov-file))
"Specifies how to find the coverage file.  Set if you have a different setup."


(defun dcoverage-color-stmts (stmtxml)
  "Take an XML row for a statement (STMTXML) and return a (line . color) pair."
  (let* ((lnumstr (elquery-prop stmtxml "num"))
         (lnum (string-to-number lnumstr))
         (color (if (equal (string-to-number (elquery-prop stmtxml "count")) 0)
                    dcoverage-uncovered-stm-color
                  dcoverage-covered-stm-color)))
    (cons lnum color)))

(defun dcoverage-color-branches (brxml)
  "Take an XML row for a branch (BRXML) and return a (line . color) pair."
  (let* ((lnumstr (elquery-prop brxml "num"))
         (lnum (string-to-number lnumstr))
         (tcount (string-to-number (elquery-prop brxml "truecount")))
         (fcount (string-to-number (elquery-prop brxml "falsecount")))
         (coveredct0 (if (equal tcount 0) 0 1))
         (coveredct  (+ coveredct0 (if (equal fcount 0) 0 1)))
         (color  (cond
                  ((equal coveredct 0) dcoverage-uncovered-stm-color)
                  ((equal coveredct 2) dcoverage-covered-stm-color)
                  (t dcoverage-part-covered-branch-color))))
    (cons lnum color)))
                  

             
                          

(defun dcoverage-mark-curr-line (color)
  "Mark the current line in specified COLOR."
  (let* ((ol (make-overlay (line-beginning-position)
                 (line-end-position )
                           (current-buffer)
                           nil
                           nil)))
    (overlay-put ol 'face (list :background color))
    (overlay-put ol 'oltype 'coveragemarker)
    (overlay-put ol 'evaporate t)))

(defun dcoverage-mark-list-of-lines (lst)
  "Takes in a list (LST) of (linenumber . color) pairs, and colors them all."
  (save-excursion
    (save-restriction
      (widen)
      (goto-char 1)
      (let ((currlinum 1))
        (dolist (item lst t)
          (let* ((linum (car item))
                 (lidelta (- linum currlinum))
                 (color (cdr item))
                 (i1 (forward-line lidelta))
                 (i2 (setq currlinum linum)))
            (dcoverage-mark-curr-line color)))))))

(defun dcoverage-clear-my-overlays ()
  "Clears all coloring put in by this package."
  (dolist (ol (overlays-in 1 (point-max)) t)
    (if (eq 'coveragemarker (overlay-get ol 'oltype))
        (delete-overlay ol))))

(defun dcoverage-color-buffer-from-list (lst)
  "Clears all coloring, then colors buffer according to LST."
  (dcoverage-clear-my-overlays)
  (dcoverage-mark-list-of-lines lst))

(defvar dcoverage-current-coverage-data (make-hash-table :test 'equal))


(defun dcoverage-merge-colors-in-list (lst)
  "Merge the line/color lists in LST, selecting the more 'severe' color when multiple apply."
  (let* ((sorted-list (sort (copy-sequence lst) (lambda (a b) (> (car a) (car b)))))
         (ordering (list dcoverage-uncovered-stm-color dcoverage-part-covered-branch-color dcoverage-covered-stm-color))
         (maybe-insert (lambda (newlst item)
                         (cond
                          ((null newlst) (list item))
                          ((equal (nth 0 item) (nth 0 (car newlst)))
                           (let*  ((lnum (car item))
                                   (c1 (cdr item))
                                   (c2 (cdr (car newlst)))
                                   (best-color (if (< (seq-position ordering c1) (seq-position ordering c2)) c1 c2)))
                             (cons (cons lnum best-color) (cdr newlst))))

                          (t (cons item newlst))))))
    (seq-reduce maybe-insert sorted-list '())))
          

(defun dcoverage-parse-file (fname)
  "Parse a clover.xml file (in FNAME) into a useable representation."
  (let* ((xml (elquery-read-file fname))
         (list-of-files (elquery-$ "file" xml)))
    (dolist (fitem list-of-files dcoverage-current-coverage-data)
      (let* ((fpath (elquery-prop fitem "path"))
             (fname (elquery-prop fitem "name"))
             (pkgname (elquery-prop (elquery-parent fitem) "name"))
             (fmetrics-lst  (elquery-$ "metrics" fitem))
             (empty-metrics-lst '(:coveredstatements 0 :coveredconditionals 0 :statements 0 :conditionals 0))
             (fmetrics (if (null fmetrics-lst) empty-metrics-lst (elquery-props (car fmetrics-lst))))
             (stmts (elquery-$ "[type=stmt]" fitem))
             (branches (elquery-$ "[type=cond]" fitem))
             (stmt-colors (mapcar 'dcoverage-color-stmts stmts))
             (branch-colors (mapcar 'dcoverage-color-branches branches))
             (all-line-colors (dcoverage-merge-colors-in-list (append stmt-colors branch-colors)))
             (data (plist-put fmetrics :dcoverage-line-colors all-line-colors))
             (finaldata (plist-put data :fully-qualified-name (concat pkgname "." fname))))
        (puthash fpath finaldata dcoverage-current-coverage-data)))))

(defun dcoverage-load-default-file()
  (interactive)
  "Load the coverage data from the default file (as determined by dcoverage-find-cov-file-fn)"
  (dcoverage-parse-file (funcall dcoverage-find-cov-file-fn)))

(defun dcoverage-color-current-buffer ()
  "Colors the current buffer based on the coverage data, reading it if needed."
  (interactive)
  (let* ((fname (buffer-file-name))
         (data (gethash fname dcoverage-current-coverage-data )))
    (when data
      (dcoverage-color-buffer-from-list (plist-get data :dcoverage-line-colors)))))

(defun dcoverage-fold-hash (fn hash start)
  "Fold FN (key valye ans) over HASH starting with START as initial value/answer."
  (let ((ans start))
    (maphash (lambda (k v) (setq ans (funcall fn k v ans))) hash)
    ans))


(defun dcoverage-build-coverage-row (fpath filedata otherrows)
  "Take the FPATH and FILEDATA from one file's results in dcoverage-parse-file and build a row for the summary, then conses it onto OTHERROWS."
  (let* ((fqname (plist-get filedata :fully-qualified-name))
         (adjname (string-remove-suffix ".java" (string-remove-prefix dcoverage-package-name-elide fqname)))
         (ign (message "fqname is %s, elide is %s adjname is %s" fqname dcoverage-package-name-elide adjname))
         (covstm (string-to-number (plist-get filedata :coveredstatements)))
         (covbr  (string-to-number (plist-get filedata :coveredconditionals)))
         (allstm (string-to-number (plist-get filedata :statements)))
         (allbr  (string-to-number (plist-get filedata :conditionals)))
         (covtot (+ covstm covbr))
         (alltot (+ allstm allbr))
         (covpct (if (equal alltot 0) 100 (/ (* 100 covtot) alltot))))
    (cons (list adjname covpct covstm allstm covbr allbr fpath)
          otherrows)))


(defun dcoverage-print-table-entry (str pad width sep )
  "Print one table entry (STR) padded (with PAD) to WIDTH and followed by SEP."
  (let* ((w0  (length str))
         (wpad (- width w0))
         (left (/ wpad 2))
         (right (- wpad left))
         (start (point))
         (ign0 (insert-char pad left))
         (ign1 (insert str))
         (ign2 (insert-char pad right))
         (end (point))
         (ign3 (insert sep)))
    (cons start end)))
                           
(defun dcoverage-print-table-row (cols propfns pad widths sep)
  "Print a row with COLS adjusted to have properties by PROPFNS with PAD padding at WIDTHS wide and split by SEP."
  (insert sep)
  (dolist (col cols t)
    (let* ((w (car widths))
           (se (dcoverage-print-table-entry col pad w sep))
           (start (car se))
           (end (cdr se))
           (pf (car propfns))
           (ign (funcall pf col start end)))
      (progn
       (setq propfns (cdr propfns))
       (setq widths (cdr widths)))))
  (insert "\n"))
  

(defun dcoverage-map-nested (fn lst)
  "Map FN over a two deep nested LST."
  (mapcar (lambda (x) (mapcar fn x)) lst))


        

(defun dcoverage-result-class-named-pressed (btn)
  "Handle press of BTN."
  (message "Button press for %s" (button-get btn 'dcovinfo))
  (save-selected-window
    (let*
        ((fname (button-get btn 'dcovinfo)))
      (find-file-other-window fname))))

(defun dcoverage-color-all-open-files (fnamelist)
  "Color all open files in FNAMELIST based on current data."
  (let ((startbuf (current-buffer)))
    (dolist (fnm fnamelist t)
      (let ((b (find-buffer-visiting fnm)))
        (when b
          (set-buffer b)
          (dcoverage-color-current-buffer))))
    (set-buffer startbuf)))

(defun dcoverage-show-coverage-results()
  "Show the coverage results in both a summary buffer and coloring of open buffers."
  (interactive)
  (let* ((resbuf (get-buffer-create "*Coverage Results*"))
         (nothing (set-buffer resbuf))
         (nothing2 (erase-buffer))
         (header-row (list "Class Name " " Total Coverage% " " Cvrd Stmts " " Ttl Stmts " " Cvrdd Brs " " Ttl Brs "))
         (dash-row   (list "-----------" "-----------------" "------------" "-----------" "-----------" "---------"))
         (nop-pf     (lambda (col start end) t))
         (nop-pfs    (list nop-pf nop-pf nop-pf nop-pf nop-pf nop-pf))
         (cov-pf     (lambda (col start end)
                       (let* ((n (string-to-number col))
                              (color (cond
                                      ((< n 50) dcoverage-poorly-covered-report-color)
                                      ((< n 100) dcoverage-moderate-covered-report-color)
                                      (t dcoverage-well-covered-report-color)))
                              (ol (make-overlay start end)))
                         (overlay-put ol 'face (list :foreground color))
                         (overlay-put ol 'evaporate t))))
         (data-row-pfs  (list cov-pf nop-pf nop-pf nop-pf nop-pf)) ; will add one more per row as we evaluate each fname
         (init-widths (mapcar (symbol-function 'length) header-row))
         ;;this makes a list of rows, where each row has
         ;; 1. fully qualified name (e.g., foo.bar.MyClass)
         ;;   -though we remove dcoverage-package-name-elide
         ;; 2. coverage percent
         ;; 3. covered statement count
         ;; 4. all statement count
         ;; 5. covered branch count
         ;; 6. all branch count
         ;; 7. file pathname
         (rows (dcoverage-fold-hash (symbol-function 'dcoverage-build-coverage-row) dcoverage-current-coverage-data '()))
         ;;sort the rows by order of increasing coverage percentage
         (sorted-rows (sort (copy-sequence rows) (lambda (a b) (< (nth 1 a) (nth 1 b)))))
         ;;turn all the row data into strings
         (str-rows (dcoverage-map-nested (lambda (v) (if (numberp v) (number-to-string v) v)) sorted-rows))
         ;;rev-row is each row reversed, so that file pathname is first.
         (rev-row (mapcar 'reverse str-rows))
         ;;build  up a list of just the file pathnames (but the list is backwards)
         (fnames-r (seq-reduce (lambda (ans r) (cons (car r) ans)) rev-row '()))
         ;;reverse that so they are in the right oder
         (fnames (reverse fnames-r))
         (orig-fnames fnames)
         (rev-data-rows (mapcar 'cdr rev-row))
         (data-rows (mapcar 'reverse rev-data-rows))
         (sum-data-elt (lambda (n)
                           (seq-reduce #'+ (mapcar (lambda (row) (nth n row)) rows) 0)))
         (total-cv-stm (funcall sum-data-elt 2))
         (total-stm (funcall sum-data-elt 3))
         (total-cv-br (funcall sum-data-elt 4))
         (total-br (funcall sum-data-elt 5))
         (total-cv (+ total-cv-stm total-cv-br))
         (total-denom (+ total-stm total-br))
         (total-pct (if (equal total-denom 0) 100 (/ (* 100 total-cv) total-denom)))
         (total-row (list "Totals" (number-to-string total-pct) (number-to-string total-cv-stm) (number-to-string total-stm) (number-to-string total-cv-br) (number-to-string total-br)))
         (per-row-widths (dcoverage-map-nested 'length data-rows))
         (col-widths (seq-reduce (lambda (currw r)  (seq-mapn 'max currw r)) per-row-widths init-widths))
         (r0 (dcoverage-print-table-row dash-row nop-pfs ?- col-widths "+"))
         (r1 (dcoverage-print-table-row header-row nop-pfs ?\s col-widths "|"))
         (r2 (dcoverage-print-table-row dash-row nop-pfs ?- col-widths "+")))
    (dolist (row data-rows t)
      (let* ((myfname (car fnames))
             (lam  (lambda (col start end)
                     (make-button start end
                                  'action 'dcoverage-result-class-named-pressed
                                  'dcovinfo myfname)))
             (pfs (cons lam data-row-pfs)))
        (dcoverage-print-table-row row pfs ?\s col-widths "|")
        (setq fnames (cdr fnames))))
    (dcoverage-print-table-row dash-row nop-pfs ?- col-widths "+")
    (dcoverage-print-table-row total-row (cons nop-pf data-row-pfs) ?\s col-widths "|")
    (dcoverage-print-table-row dash-row nop-pfs ?- col-widths "+")
    (insert "\n\n")
    (insert-button "Remove Colors From All Buffers" 'action (lambda (btn) (dcoverage-clear-all)))
    (insert "\n")
    (dcoverage-color-all-open-files orig-fnames)
    (switch-to-buffer-other-window resbuf)))

(defun dcoverage-run-clover()
  "Rename clover.xml file to have .old, then run the build command."
  (interactive)
  (let* ((fname (dcoverage-default-find-cov-file))
         (newfname (concat fname ".old"))
         (ign  (if (file-exists-p fname)
                   (rename-file fname newfname t)
                 t)))
    (funcall dcoverage-run-clover-function)))

(defun dcoverage-generate-and-show()
  "Generate the coverage results, and show them."
  (interactive)
  (setq compilation-exit-message-function (lambda (pstat estat emesg)
                                            (if (equal estat 0)
                                                (progn (dcoverage-load-default-file)
                                                       (dcoverage-show-coverage-results))
                                              t)
                                            (setq compilation-exit-message-function nil)
                                            (cons emesg "")))
                                              
                                              
  (dcoverage-run-clover))


(setq dcoverage-save-to-file-name "cov.txt")

;;this is a hack to let us run dcoverage-generate-and-save
;;from a script.  Since the compilation is async, the script
;;would just exit.  We can make the script wait for
;;this variable to become true.  Would be nice
;;to use mutex/cv but those don't exist until Emacs 26.
(setq dcoverage-save-done nil)


(defun dcoverage-generate-and-save(fname)
  "Generates the coverage results, and saves them to a file named FNAME."
  (interactive)
  (setq dcoverage-save-to-file-name fname)
  (setq dcoverage-save-done nil)
  (setq compilation-exit-message-function (lambda (pstat estat emesg)
                                            (if (equal estat 0)
                                                (progn (dcoverage-load-default-file)
                                                       (dcoverage-show-coverage-results)
                                                       ;;show-coverage-results leaves us in results buffer
                                                       (write-file dcoverage-save-to-file-name))
                                              t)
                                            (setq dcoverage-save-done t)
                                            (cons emesg "")))
                                              
                                              
  (dcoverage-run-clover))


(defun dcoverage-clear-all()
  "Clears coloration from all open buffers and drops coverage data."
  (interactive)
  (setq dcoverage-current-coverage-data (make-hash-table :test 'equal))
  (let ((startbuf (current-buffer))
        (blist (buffer-list)))
    (dolist (buf blist t)
      (set-buffer buf)
      (dcoverage-clear-my-overlays))
    (set-buffer startbuf)))
        
(global-set-key dcoverage-key-generate-and-show 'dcoverage-generate-and-show)
(global-set-key dcoverage-key-clear-all 'dcoverage-clear-all)


(add-hook 'find-file-hook 'dcoverage-color-current-buffer)

(provide 'dcoverage)
;;; dcoverage.el ends here




EOF

cat > ~/.emacs <<EOF
(require 'package)
(add-to-list 'package-archives '("melpa" . "http://melpa.org/packages/"))
(package-initialize)
(require 'use-package)

;; Uncomment this next line if you want line numbers on the left side
(global-linum-mode 1)
(global-set-key "\C-c\C-v" 'compile)
(setq line-number-mode t)
(setq column-number-mode t)
(display-time)
(global-font-lock-mode t)
(setq font-lock-maximum-decoration t)

;;This makes rainbow delimiters mode the default.
;;comment out to turn it off.
(add-hook 'find-file-hook 'rainbow-delimiters-mode-enable)

;;Want electric pair mode? Uncomment the next line
;(electric-pair-mode)

;;Want to turn off show paren mode? Comment out the below line.
(show-paren-mode)


(custom-set-variables
 ;; custom-set-variables was added by Custom.
 ;; If you edit it by hand, you could mess it up, so be careful.
 ;; Your init file should contain only one such instance.
 ;; If there is more than one, they won't work right.
 '(column-number-mode t)
 '(compilation-auto-jump-to-first-error nil)
 '(dap-auto-show-output nil)
 '(dap-java-test-additional-args (quote ("--include-classname" ".+")))
 '(dcoverage-moderate-covered-report-color "dark orange")
 '(dcoverage-package-name-elide "edu.duke.ece651.")
 '(dcoverage-pooly-covered-report-color "red")
 '(dcoverage-well-covered-report-color "green")
 '(display-time-mode t)
 '(inhibit-startup-screen t)
 '(lsp-java-format-on-type-enabled nil)
 '(package-selected-packages
   (quote
    (lsp-ivy use-package helm-lsp dracula-theme posframe lsp-ui lsp-mode groovy-mode forge magit memoize lsp-javacomp lsp-java jtags javadoc-lookup java-imports gradle-mode flycheck-popup-tip flycheck-gradle company-rtags company-c-headers clang-format)))
 '(safe-local-variable-values (quote ((TeX-master . t))))
 '(show-paren-mode t))




(custom-set-faces
 ;; custom-set-faces was added by Custom.
 ;; If you edit it by hand, you could mess it up, so be careful.
 ;; Your init file should contain only one such instance.
 ;; If there is more than one, they won't work right.
 '(default ((t (:family "Ubuntu Mono" :foundry "DAMA" :slant normal :weight normal :height 120 :width normal))))
 '(lsp-ui-peek-line-number ((t (:foreground "deep sky blue"))))
 '(lsp-ui-peek-selection ((t (:background "blue" :foreground "white smoke"))))
 '(lsp-ui-sideline-code-action ((t (:background "black" :foreground "lawn green")))))


(global-set-key "\C-x\C-g" 'goto-line)


; Automatically set compilation mode to
; move to an error when you move the point over it
;(add-hook 'compilation-mode-hook
; (lambda () 
;   (progn
;     (next-error-follow-minor-mode))))

;;Automatically go to the first error
;;This works great for C/C++---not so much for Java
;; (results in a lot of trying to find the file, since the full path
;; isn't usually in the error message)
;(setq compilation-auto-jump-to-first-error t)
(setq-default indent-tabs-mode nil)


(use-package company)
;(global-company-mode)
(use-package flycheck)
;(global-flycheck-mode)
;(flycheck-popup-tip-mode)

;;This is from 551, where we had grade.txt in color.
;; (defun colorize-grade-txt ()
;;   "Make a grade.txt file show colors, then set read only."
;;   (interactive)
;;   (if (or (string= (buffer-name) "grade.txt")
;;           (string-prefix-p "grade.txt<" (buffer-name)))
;;       (progn (let ((inhibit-read-only t))
;;                (ansi-color-apply-on-region (point-min) (point-max)))
;;              (set-buffer-modified-p nil)
;;              (read-only-mode t))))

;; (add-hook 'find-file-hook 'colorize-grade-txt)


(add-hook 'gud-mode-hook (lambda() (company-mode 0)))

(setq gdb-many-windows t
      gdb-use-separate-io-buffer t)
(advice-add 'gdb-setup-windows :after (lambda() (set-window-dedicated-p (selected-window) t)))

(defconst gud-windown-register 123456)

(defun gud-quit()
  (interactive)
  (gud-basic-call "quit"))

(add-hook 'gud-mode-hook
          (lambda()
            (gud-tooltip-mode)
            (window-configuration-to-register gud-windown-register)
            (local-set-key (kbd "C-q") 'gud-quit)))

(advice-add 'gud-sentinel :after
            (lambda (proc msg)
              (when (memq (process-status proc) '(signal exit))
                (jump-to-register gud-windown-register)
                (bury-buffer))))
            


(use-package gradle-mode)
(use-package lsp-mode
  :init
  (setq lsp-keymap-prefix "C-c l")
  :hook
  ((java-mode . lsp))
  :commands lsp
  )
(use-package lsp-ui :commands lsp-ui-mode)
(use-package helm-lsp :commands helm-lsp-workspace-symbol)
(use-package lsp-ivy :commands lsp-ivy-workspace-symbol)
(use-package lsp-treemacs :commands lsp-treemacs-errors-list)
(use-package dap-mode)
(use-package dap-java)
;(use-package lsp-java)


(defun find-path-component(path target)
  "Find TARGET in PATH, returning a list of all components found along the way. 
Returns t if TARGET not found."
  (cond ((equal path "") t)
        ((equal target (file-name-nondirectory path)) nil)
        ((equal path "/") t)
        (t (let ((tmp (find-path-component (directory-file-name (file-name-directory path)) target)))
             (if (equal tmp t)
                 t
               (cons (file-name-nondirectory path) tmp))))))

(defun java-smart-class-skel ()
  "Generate a Java class skeleton based on the current path."
  (interactive)

  (let* ((bname (buffer-file-name))
         (cname (file-name-base bname))
         (ctype (find-path-component bname "src"))
         (istest (if (listp ctype) (equal (car (reverse ctype)) "test") nil))
         (pkg (find-path-component bname "java")))
    (if (and (listp pkg)
             (> (length pkg) 1))
        (progn
          (insert "package ")
          (insert (mapconcat 'identity (reverse (cdr pkg)) "."))
          (insert ";\n\n")))
    (if istest
        (progn
          (insert "import static org.junit.jupiter.api.Assertions.*;\n\n")
          (insert "import org.junit.jupiter.api.Test;\n\n")))
    (insert "public class " cname " ")
    (insert "{\n")
    (if istest
          (insert "  @Test\n  public void test_() {\n\n  }\n"))
    (insert "\n")
    (insert "}\n")
    (if istest
        (progn
          (search-backward "public void test_()")
          (search-forward "()")
          (backward-char 2))
      
      (progn
        (goto-char (point-min))
        (search-forward (concat "public class " cname))))
    (if (buffer-file-name) (save-buffer))))
(use-package hydra)        



(use-package elquery)
(add-to-list 'load-path "~/.emacs.d/dcoverage/")
(use-package dcoverage)
(use-package yasnippet)
(yas-global-mode)
(defun build-and-run ()
  "Single key stroke for gradle to build and run the program."
  (interactive)
  (gradle-run "--info build run"))

(define-key gradle-mode-map (kbd "C-c C-r") 'build-and-run)

(add-to-list 'auto-mode-alist '("build.gradle" . groovy-mode))


(use-package magit
  :defer 2
  :config (global-set-key "\C-cg" 'magit-status))
(use-package forge
  :defer 5
  :config (add-to-list 'forge-alist '("gitlab.oit.duke.edu"  "gitlab.oit.duke.edu/api/v4" "gitlab.oit.duke.edu" forge-gitlab-repository)))

(add-hook 'dap-stopped-hook
          (lambda (arg)
            (call-interactively #'dap-hydra)))

(global-set-key "\C-c\C-h" 'hydra-pause-resume)

(add-hook 'dap-terminated-hook
           (lambda (arg)
             (hydra-disable)))




(defun gradle-clean-and-build ()
  "Run gradle clean build."
  (interactive)
  (gradle-run "clean classes testClasses"))

(defun ece651-debug-test-case()
  "Ensure that classes are up to date, copy to bin, and debug current test."
  (interactive)
  ;;we'll make sure everything is up to date
  (let* ((basedir (dcoverage-find-project-root))
         (testdir (f-join basedir "build" "classes" "java" "test"))
         (maindir (f-join basedir "build" "classes" "java" "main"))
         (cloverdir (f-join maindir "clover.instrumented"))
         (bindir (f-join basedir "bin"))
         (testdest (f-join bindir "test"))
         (maindest (f-join bindir "main")))
    ;;check that build made these before continuing
    (if (not (file-directory-p testdir))
        (error "Test directory %s does not exist (run gradle?)" testdir))
    (if (not (file-directory-p maindir))
        (error "Main directory %s does not exist (run gradle?)" maindir))
    (if (file-directory-p cloverdir)
        (error "It looks like your code is instrumented.  Run gradle-clean-and-build first!"))
    ;;copy build dirs to bin/
    (delete-directory testdest t)
    (delete-directory maindest t)
    (copy-directory testdir testdest t t nil)
    (copy-directory maindir maindest t t nil)
    ;;now we can run the debugger
    (message "Starting debugger... %s" (current-buffer))
    (dap-breakpoint-add)
    (save-window-excursion (call-interactively 'dap-java-debug-test-method))
    (message "Press C-c C-h to toggle debug hydra")
    (cons "debugger started" "debugging")))

    
;ctype should be '(classbuilder java main test)
(defun toggle-code-to-test-buffer()
  "Switch between code and test for a given class.   If the test code doesn't exist, init it."
  (interactive)
  (let* ((prjroot (dcoverage-find-project-root))
         (bname (buffer-file-name))
         (fext   (file-name-extension bname))
         (cname (file-name-base bname))
         (ctype (find-path-component bname "src"))
         (istest (if (listp ctype) (equal (car (reverse ctype)) "test") nil))
         (pkg (find-path-component bname "java")))
    (if (not (equal fext "java"))
        (error "%s is not a .java file" bname))
    (if (and istest (not (string-suffix-p "Test" cname)))
        (error "%s is in test directory but is not named (something)Test.java"))
    (let* ((newname (if istest
                         (substring cname 0 -4)  ;; test -> non test: remove Test from name
                       (concat cname "Test")))   ;; non test -> test add Test to name
           (testormain (if istest "main" "test"))  ;; swap test/main names
            ;;now we need prjroot/src/[test|main]/(all the stuff in cdr ctype)/newname.java
           (packagename (apply 'f-join (cdr (reverse (cdr ctype)))))
           (ign (message "(f-join %s %s %s %s)" prjroot "src" testormain packagename))
           (theotherdir (f-join prjroot "src" testormain packagename ))
           (ignored (make-directory theotherdir t))
           (theotherfile (f-join theotherdir (concat newname ".java")))
           (ignored (message "Switching to %s " theotherfile))
           (thebuffer (find-file theotherfile)))
      (if (= (buffer-size thebuffer) 0)
          (java-smart-class-skel)))))
            


(add-hook 'java-mode-hook
          (lambda()
            (flycheck-mode +1)
            (setq c-basic-offset 2)
            (gradle-mode)
            (dap-mode 1)
            (dap-ui-mode 1)
            (dap-tooltip-mode 1)
            (tooltip-mode 1)
            (if (not window-system)
                (setq dap-auto-configure-features (remove 'controls dap-auto-configure-features)))
            ;(dap-ui-controls-mode 1)
            ;(lsp)
            (local-set-key "\C-c\C-a" 'lsp-java-add-unimplemented-methods)
            (local-set-key "\C-c\C-i" 'lsp-java-organize-imports)
            (local-set-key "\C-c\C-o" 'lsp-java-generate-overrides)
            (local-set-key "\M-gg" 'lsp-java-generate-getters-and-setters)
            (local-set-key "\C-c\C-e" 'lsp-java-extract-method)
            (local-set-key "\C-ci" 'lsp-goto-implementation)
            (local-set-key "\C-ct" 'lsp-goto-type-definition)
            (local-set-key "\C-c\C-j" 'javadoc-lookup)
            (local-set-key "\C-c\C-v" 'gradle-execute)
            (local-set-key "\C-c\C-f" 'lsp-format-buffer)
            (local-set-key "\C-cr" 'lsp-rename)
            (local-set-key "\C-cd" 'lsp-ui-peek-find-definitions)
            (local-set-key "\C-cu" 'lsp-ui-peek-find-references)
            (local-set-key "\C-cx"  'gradle-clean-and-build)
            (local-set-key "\C-c\C-s" 'java-smart-class-skel)
            (local-set-key "\C-c\C-d" 'ece651-debug-test-case)
            (local-set-key "\C-xt" 'toggle-code-to-test-buffer)
            (setq tab-width 2)))
                 
(add-hook 'latex-mode-hook 'flyspell-mode)
(add-hook 'latex-mode-hook 'flyspell-buffer)

(load-theme 'dracula t)




EOF

echo "Checking for clang-format"
x=`which clang-format`
if [ "$x" != "" ]
then
    echo "Found in $x"
    echo "Setting up emacs to use for C/C++ on save"
    if [ -f ~/.clang-format ]
    then
        echo "~/.clang-format exists.  Not replacing"
    else
        echo "Using 551 clang format"
        cp clang-format ~/.clang-format
    fi
    cat >> ~/.emacs <<EOF
(use-package clang-format)


(add-hook 'c-mode-hook
          (function (lambda ()
                    (add-hook 'write-contents-functions
                              (lambda() (progn (clang-format-buffer) nil))))))

(add-hook 'c++-mode-hook
          (function (lambda ()
                      (add-hook 'write-contents-functions
                                (lambda() (progn (clang-format-buffer) nil))))))
EOF
else
    echo "No clang-format found."
fi
    

if [ "$EDITOR" == "emacs" ]
then
    echo "It looks like EDITOR is already emacs!"
else
    export EDITOR='emacs'
    echo "EDITOR='emacs'" >> ~/.bashrc
    export VISUAL='emacs'
    echo "VISUAL=emacs" >> ~/.bashrc 
fi

