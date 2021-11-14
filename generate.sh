#!/usr/bin/env bash
set -e

rm -rf bindings/ 2> /dev/null || true
cargo test --tests --features=ts-rs

targetfile='static/typings.ts'
cat typescript/manual.ts > $targetfile

for file in bindings/*.ts ; do
    # 1: remove imports as everything is in the same file
    # 2: readonly properties
    # 3: readonly array values
    # 4: Readonly<Partial<Record<K, V>>>
    # 5: bla | null -> bla | undefined
    # 6: bigint -> number
    # last: remove empty lines
    sed -E \
        -e "s#^import.+##g" \
        -e "s#^  (\w+:)#  readonly \1#g" \
        -e "s#Array#ReadonlyArray#g" \
        -e "s#Record<([^>]+)>#Readonly<Partial<Record<\1>>>#g" \
        -e "s#: (\S+) \| null;#\?: \1;#g" \
        -e "s#bigint#number#g" \
        -e "/^$/d" \
        "$file" >> $targetfile
done

# Add final newline
printf "\n" >> $targetfile
