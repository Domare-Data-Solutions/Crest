
description=""

GREEN='\033[1;32m'
RED='\033[0;31m'
NC='\033[0m'

declare -g FILE

add_flag '-' "file" "read a file instead of an argument" 1
function flag_name_file {
    FILE="-f"
}

add_argument "rule" "string" "the rule type to check against"
add_argument "source" "string..." "the source to check"

function target_validate_grammar {
    local rule="$1"
    shift
    local source="$*"

    clear
    [[ -z ${FILE} ]] && echo -e "${RED}${rule}${NC}: ${GREEN}${source}${NC}\n==================="
    [[ -n ${FILE} ]] && echo -e "${RED}${rule}${NC}:\n${GREEN}$(cat ${source})${NC}\n==================="
    if ! cargo run -q -j 4 --example validate-grammar -- ${rule} ${FILE} "${source}"; then
        echo
    fi

    local grammar_hash=$(sha256sum src/css.pest | awk '{print $1}')
    local validator_hash=$(sha256sum examples/validate-grammar.rs | awk '{print $1}')
    sleep 1

    while true; do
        local new_grammar_hash=$(sha256sum src/css.pest | awk '{print $1}')
        local new_validator_hash=$(sha256sum examples/validate-grammar.rs | awk '{print $1}')
        if [[ ${new_grammar_hash} != ${grammar_hash} || ${new_validator_hash} != ${validator_hash} ]]; then
            grammar_hash="${new_grammar_hash}"
            validator_hash="${new_validator_hash}"
            clear
            [[ -z ${FILE} ]] && echo -e "${RED}${rule}${NC}: ${GREEN}${source}${NC}\n==================="
            [[ -n ${FILE} ]] && echo -e "${RED}${rule}${NC}:\n${GREEN}$(cat ${source})${NC}\n==================="
            if ! cargo run -q -j 4 --example validate-grammar -- ${rule} ${FILE} "${source}"; then
                echo
            fi
        fi
        sleep 1
    done
}
