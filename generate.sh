#!/usr/bin/env bash
set -e

rm ./*.ts 2> /dev/null || true
cargo test --tests --all-features

targetfile='static/typings.ts'
cat typescript/manual.ts > $targetfile

for file in typescript/generated-*.ts ; do
    printf "\n" >> $targetfile
    # 1: remove imports as everything is in the same file
    # 2: readonly properties
    # 3: readonly array values
    # 4: Readonly<Partial<Record<K, V>>>
    # 5: bla | null -> bla | undefined
    # 6: : null -> ?: null
    # 7: remove empty lines
    sed -E \
        -e "s#^import.+##g" \
        -e "s#^  (\w+:)#  readonly \1#g" \
        -e "s#: (.+)\[\];\$#: readonly \1[];#g" \
        -e "s#\{ \[key: (\S+)\]: (\S+) }#Readonly<Partial<Record<\1, \2>>>#g" \
        -e "s#: (\S+) \| null;#\?: \1;#g" \
        -e "s#: null#\?: null#g" \
        -e "/^$/d" \
        "$file" >> $targetfile
    rm "$file"
done

# Add final newline
printf "\n" >> $targetfile
