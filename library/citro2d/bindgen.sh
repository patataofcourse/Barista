#!/usr/bin/env bash

#TODO: https://rust-lang.github.io/rust-bindgen/library-usage.html

set -euxo pipefail

bindgen "$DEVKITPRO/libctru/include/citro2d.h" \
    --rust-target nightly \
    --use-core \
    --distrust-clang-mangling \
    --no-doc-comments \
    --no-layout-tests \
    --ctypes-prefix "::libc" \
    --no-prepend-enum-name \
    --fit-macro-constant-types \
    --must-use-type "Result" \
    --generate "functions,types,vars" \
    --blocklist-type "u(8|16|32|64)" \
    --blocklist-type "__builtin_va_list" \
    --blocklist-type "__va_list" \
    --opaque-type "MiiData" \
    -- \
    --target=arm-none-eabi \
    --sysroot=$DEVKITARM/arm-none-eabi \
    -isystem$DEVKITARM/arm-none-eabi/include \
    -I$DEVKITPRO/libctru/include \
    -mfloat-abi=hard \
    -march=armv6k \
    -mtune=mpcore \
    -mfpu=vfp \
    -D__3DS__ \
> src/bindings.rs