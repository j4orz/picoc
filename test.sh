#!/bin/bash

assert() {
  input="$1"
  expected="$2"

  ./target/release/din "$input" > tmp.s || exit
  riscv64-unknown-elf-gcc -o tmp tmp.s
  spike pk tmp
  actual="$?"

  if [ "$expected" = "$actual" ]; then
    echo "$input => $actual"
  else
    echo "$input => expected $expected, but got $actual"
    exit 1
  fi
}

assert "./tests/fixtures/din/legal/arithmetic/lit.c" 8
assert "./tests/fixtures/din/legal/arithmetic/add.c" 19
assert "./tests/fixtures/din/legal/arithmetic/add_multi.c" 30
assert "./tests/fixtures/din/legal/arithmetic/sub.c" 56

echo OK