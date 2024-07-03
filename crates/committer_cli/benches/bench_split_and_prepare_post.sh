# Benchmark the new code, splitting the benchmarks
# TODO: split the output file instead.
cat ${1} |
    while read line; do
        cargo bench $line > ${line}.txt;
        sed -i '/'"${line}"'/,$!d' ${line}.txt;
    done

# Prepare the results for posting comment.
echo "Benchmark movements:" > ${2}
cat ${1} |
    while read line; do
        if grep -q "regressed" ${line}.txt; then
            echo "**${line} performance regressed!**" >> ${2};
            cat ${line}.txt >> ${2};
        elif grep -q "improved" ${line}.txt; then
            echo "_${line} performance improved_ :smiley_cat:" >> ${2};
            cat ${line}.txt >> ${2};
        fi;
    done
