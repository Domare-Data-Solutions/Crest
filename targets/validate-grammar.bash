
description=""

GREEN='\033[1;32m'
RED='\033[0;31m'
NC='\033[0m'

delcare -ga WATCH_FILES
declare -gA HASHES
declare -g EXTRA_ARGS
declare -g FILE=0

function record_hash {
    local file="$1"
    local hash=$(sha256sum ${file} | awk '{print $1}')
    ${HASHES[${file}]}=${hash}
}

function check_hash {
    local file="$1"
    local record_on_fail="$2"
    local hash_current="$(get_hash ${file})"
    local hash_record="${HASHES[${file}]}"
    if [[ ${hash_current} == ${hash_record} ]]; then
        echo y
    else
        echo n
    fi
    [[ ${record_on_fail} == y ]] && ${HASHES[${file}]}=${hash_current}
}

add_flag '-' "file" "read a file instead of an argument" 1
function flag_name_file {
    EXTRA_ARGS+="-f "
    FILE=1
}

add_flag '-' "extra-args" "extra args to pass to the validate-grammar example" 2 "args" "string"
function flag_name_extra_args {
    EXTRA_ARGS+="$1 "
}

add_argument "rule" "string" "the rule type to check against"
add_argument "source" "string..." "the source to check"

function target_validate_grammar {
    local rule="$1"
    shift
    local source="$*"

    clear
    [[ ${FILE} -eq 0 ]] && echo -e "${RED}${rule}${NC}: ${GREEN}${source}${NC}\n==================="
    [[ ${FILE} -eq 1 ]] && echo -e "${RED}${rule}${NC}:\n${GREEN}$(cat ${source})${NC}\n==================="
    if ! cargo run -q -j 4 --example validate-grammar -- ${EXTRA_ARGS} ${rule} "${source}"; then
        echo
    fi

    for file in "${WATCH_FILES[@]}"; do
        record_hash ${file}
    done
    sleep 1

    while true; do
        for file in "${WATCH_FILES[@]}"; do
            if [[ $(check_hash ${file}) == n ]]; then
                clear
                [[ -z ${EXTRA_ARGS} ]] && echo -e "${RED}${rule}${NC}: ${GREEN}${source}${NC}\n==================="
                [[ -n ${EXTRA_ARGS} ]] && echo -e "${RED}${rule}${NC}:\n${GREEN}$(cat ${source})${NC}\n==================="
                if ! cargo run -q -j 4 --example validate-grammar -- ${EXTRA_ARGS} ${rule} "${source}"; then
                    echo
                fi
                break
            fi
        done
        sleep 1
    done
}
