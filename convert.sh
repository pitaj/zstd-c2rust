#!/bin/bash
set -e

# Run cmake to produce a compilation database
mkdir -p build/cmake/output
cd build/cmake/output
cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=ON -DCMAKE_BUILD_TYPE=Debug ..

# Transpile with c2rust
cd ../../..
c2rust transpile --emit-no-std --emit-build-files --overwrite-existing --reduce-type-annotations --translate-const-macros \
  --output-dir . build/cmake/output/compile_commands.json 2>&1 | tee >(sed $'s/\033[[][^A-Za-z]*m//g' > c2rust.log)

mv -f src/lib/common/* src/common
mv -f src/lib/compress/* src/compress
mv -f src/lib/decompress/* src/decompress
mv -f src/lib/dictBuilder/* src/dict_builder
mv -f src/lib/legacy/* src/legacy
rmdir src/lib/*

sed -i "2 s/use ::c2rust_bitfields;/use ::c2rust_bitfields::BitfieldStruct;/" src/compress/zstdmt_compress.rs
sed -i "1i use crate::__m128i_u;" src/compress/zstd_compress.rs src/compress/zstd_double_fast.rs src/compress/zstd_fast.rs \
  src/compress/zstd_lazy.rs src/compress/zstd_ldm.rs src/compress/zstd_opt.rs src/decompress/zstd_decompress_block.rs

# Replace intrinsics with inherent methods
# ::core::intrinsics::rotate_(left|right)\([\s\n]*([^,]+)[\s\n]*,[\s\n]*([^\),]+?[\s\n]+as[\s\n]+libc::c_int[\s\n]+as[\s\n]+)libc::c_u\w+,?[\s\n]*\)
# $2.rotate_$1($3u32)
