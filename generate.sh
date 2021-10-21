#!/usr/bin/env bash
set -e

rm ./typescript/generated-*.ts 2> /dev/null || true
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
    # last: remove empty lines
    sed -E \
        -e "s#^import.+##g" \
        -e "s#^  (\w+:)#  readonly \1#g" \
        -e "s#Array#ReadonlyArray#g" \
        -e "s#Record<([^>]+)>#Readonly<Partial<Record<\1>>>#g" \
        -e "s#: (\S+) \| null;#\?: \1;#g" \
        -e "/^export type SitesNearPlanet/d" \
        -e "/^export type Storage/d" \
        -e "/^$/d" \
        "$file" >> $targetfile
    rm "$file"
done

# Add final newline
printf "\n" >> $targetfile
