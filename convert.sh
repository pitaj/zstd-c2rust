#!/bin/bash
set -e

case $1 in

  transpile)
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
    rmdir src/lib

    ;;

  missing-imports)
    # Fix missing imports
    sed -i "2 s/use ::c2rust_bitfields;/use ::c2rust_bitfields::BitfieldStruct;/" src/compress/zstdmt_compress.rs
    sed -i "1i use crate::__m128i_u;" src/compress/zstd_compress.rs src/compress/zstd_double_fast.rs src/compress/zstd_fast.rs \
      src/compress/zstd_lazy.rs src/compress/zstd_ldm.rs src/compress/zstd_opt.rs src/decompress/zstd_decompress_block.rs

    # Replace intrinsics with inherent methods
    # ::core::intrinsics::rotate_(left|right)\([\s\n]*([^,]+)[\s\n]*,[\s\n]*([^\),]+?[\s\n]+as[\s\n]+libc::c_int[\s\n]+as[\s\n]+)libc::c_u\w+,?[\s\n]*\)
    # -> $2.rotate_$1($3u32)

    ;;

  asserts)
    # Replace MIN(MAX()) with .clamp
    # (if 4 as libc::c_int as libc::c_uint
    #         > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
    #             6 as libc::c_int as libc::c_uint
    #         } else {
    #             (*ms).cParams.minMatch
    #         })
    #     {
    #         4 as libc::c_int as libc::c_uint
    #     } else {
    #         (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.minMatch {
    #             6 as libc::c_int as libc::c_uint
    #         } else {
    #             (*ms).cParams.minMatch
    #         })
    #     })
    # -> (*ms).cParams.minMatch.clamp(4, 6)
    # (if 4 as libc::c_int as libc::c_uint
    #         > (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
    #             6 as libc::c_int as libc::c_uint
    #         } else {
    #             (*ms).cParams.searchLog
    #         })
    #     {
    #         4 as libc::c_int as libc::c_uint
    #     } else {
    #         (if (6 as libc::c_int as libc::c_uint) < (*ms).cParams.searchLog {
    #             6 as libc::c_int as libc::c_uint
    #         } else {
    #             (*ms).cParams.searchLog
    #         })
    #     })
    # -> (*ms).cParams.searchLog.clamp(4, 6)

    # Replace code from C `assert(...)` with `debug_assert!(...)`
    perl -i -p0e 's/^( *)if[\s\n]+([^\{]*?(?:\n\1 +\{[^\}]*?(?:\} else \{[^\}]*?)\n\1 +\}[^\{]*?)?)[\s\n]*\{\} else \{\n\1    __assert_fail\([\s\n]*([^,]+)[^\}]*?\1\}/$1debug_assert!($2);/gm' src/*/*.rs

    # ^( *)__assert_fail\([\s\n]*b"0\\0"[\s\S]*?\n\1\);[\s\n]*?( *unreachable!\(\);)
    # -> $2;

    # ^( *)__assert_fail\([\s\n]*b"0\\0"[\s\S]*?\n\1\);
    # -> $1debug_assert!(false);

    # 
    #     fn __assert_fail(
    #         __assertion: *const libc::c_char,
    #         __file: *const libc::c_char,
    #         __line: libc::c_uint,
    #         __function: *const libc::c_char,
    #     ) -> !;
    # -> 

    ;;

  integers)
    # Replace integer types with rust equivalents

    perl -i -p0e 's/\bU(64|32|16|8)\b/u$1/gm' src/*/*.rs
    perl -i -p0e 's/\bS(64|32|16|8)\b/i$1/gm' src/*/*.rs
    perl -i -p0e 's/\b(?:__)?(?:(u)i|(i))nt(64|32|16|8)_t\b/$1$2$3/gm' src/*/*.rs

    perl -i -p0e 's/\bBYTE\b/u8/gm' src/*/*.rs
    perl -i -p0e 's/\b(?:libc::)?size_t\b/libc::size_t/gm' src/*/*.rs

    # Remove old typedefs
    perl -i -p0e 's/\npub type (u8|u16|u32|u64|usize|i8|i16|i32|i64|isize) = [^;]+;//gm' src/*/*.rs
    perl -i -p0e 's/\npub type (?:libc::)?size_t = [^;]+;//gm' src/*/*.rs

    # Remove redundant as casts
    perl -i -p0e 's/\b(as \w\S*) \1\b//gm' src/*/*.rs
    perl -i -p0e 's/(.wrapping_(?:sub|add|div|mul|rem)\(\d+)(?: as \w\S*?)+\)/$1)/gm' src/*/*.rs

    ;;

  *)
    echo "Unknown stage `$1`. Available stages:"
    echo "  transpile, missing-imports, asserts, integers"
    exit 1

    ;;

esac