#!/usr/bin/env bash
# [bash スクリプトの先頭によく書く記述のおさらい | Money Forward Engineers' Blog](https://moneyforward.com/engineers_blog/2015/05/21/bash-script-tips/)
# [使いやすいシェルスクリプトを書く | SOTA](http://deeeet.com/writing/2014/05/18/shell-template/)

set -ue -o pipefail

function usage {
    cat <<EOF
$(basename "$0") is a tool for ...

Usage:
    $(basename "$0") [command] [<options>]

Options:
    --help, -h        print this
EOF
}

case $1 in
    *)
        usage
        exit 1
    ;;
esac
