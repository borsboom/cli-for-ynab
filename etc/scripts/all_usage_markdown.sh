#!/usr/bin/env bash
set -eu -o pipefail

cd "$(dirname "$0")/../.."

temp=$(mktemp)
trap 'rm -f "$temp"' EXIT

convert_help() {
	$1 --help 2>&1|tail +2|sed -e 's/^USAGE:$/##### Usage\
/' -e 's/^FLAGS:$/##### Flags/' -e 's/^OPTIONS:$/##### Options/' -e 's/\(YNAB_[A-Z_]*=\)[^]]*/\1/' -e 's/^     *\(-.*\)/\
  * **`\1`**  /' -e 's/^            \(.*\)/    \1/' -e 's/`ynab --help`/[Global arguments](#global-arguments)/'
}

do_subs() {
	local subs=$($1 --help 2>&1|grep -A 9999 '^SUBCOMMANDS:'|grep '^    [a-z-]\+'|awk '{print $1}')
	if [[ -z $subs ]]; then
		echo ""
		echo "### $1"
		echo ""
		convert_help "$1" >"$temp"
		grep -B 9999 '^##### Flags' "$temp"|grep -v '^##### Flags'
		grep -A 9999 '^##### Options' "$temp"
	fi
	for sub in $subs; do
		do_subs "$1 $sub"
	done
}

echo "## Global arguments"
echo
echo "These arguments are accepted by all subcommands, and may also appear before the subcommand."
echo
#@@@ MAKE SURE THIS USES THE RIGHT VERSION OF YNAB
convert_help "ynab"|grep -A 9999 '^##### Flags'|grep -B 9999 '^SUBCOMMANDS:$'|grep -v '^SUBCOMMANDS:$'

echo
echo "## Commands"
echo
do_subs "ynab"
