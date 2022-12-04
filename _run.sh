#!/bin/sh

DAY=${1:-XX}

RS="day${DAY}.rs"
EXEC=solution
OUT="day${DAY}.out"

rustc -o "${EXEC}" "${RS}" || exit 1

./"${EXEC}" > "${OUT}" 
exec_error=$?

cat "${OUT}"
[ $exec_error -ne 0 ] &&  rm "${OUT}"

rm ${EXEC}

exit $exec_error 
