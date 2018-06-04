; vi:ft=nasm
; dictionary structure
;   - pointer to next entry
;   - key (null terminated string)
;   - value
bits 64

%define list_head 0

%macro colon 2
%2:
dq list_head
db %1, 0
%define list_head %2
%endmacro
