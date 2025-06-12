#!/bin/bash
set -e

case $1 in

  clean)
    rm -rf src
    rm -f Cargo.toml
    rm -f Cargo.lock
    rm -f build.rs
    rm -f lib.rs
    rm -f rust-toolchain.toml

    ;;

  transpile)
    # Run cmake to produce a compilation database
    mkdir -p build/cmake/output
    cd build/cmake/output
    cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=ON -DCMAKE_BUILD_TYPE=Debug ..

    # Transpile with c2rust
    cd ../../..
    c2rust transpile --emit-no-std --emit-build-files --overwrite-existing --reduce-type-annotations --translate-const-macros \
      --output-dir . build/cmake/output/compile_commands.json 2>&1 | tee >(sed $'s/\033[[][^A-Za-z]*m//g' > c2rust.log)

    ;;

  file-structure)
    mv -f src/lib/common/ src/common
    mv -f src/lib/compress/ src/compress
    mv -f src/lib/decompress/ src/decompress
    mv -f src/lib/dictBuilder/ src/dict_builder
    mv lib.rs src/lib.rs
    rm -rf src/lib
    rm -rf src/programs
    rm build.rs
    sed -i '/path = "lib.rs"/d' Cargo.toml
    ;;

  missing-m128i_u)
    sed -i "1i use crate::__m128i_u;" src/compress/zstd_compress.rs src/compress/zstd_double_fast.rs src/compress/zstd_fast.rs \
      src/compress/zstd_lazy.rs src/compress/zstd_ldm.rs src/compress/zstd_opt.rs src/decompress/zstd_decompress_block.rs

    ;;

  integers)
    # Replace integer types with rust equivalents

    perl -i -p0e 's/\bU(64|32|16|8)\b/u$1/gm' src/*/*.rs
    perl -i -p0e 's/\bS(64|32|16|8)\b/i$1/gm' src/*/*.rs
    perl -i -p0e 's/\b(?:__)?(?:(u)i|(i))nt(64|32|16|8)_t\b/$1$2$3/gm' src/*/*.rs

    perl -i -p0e 's/\bBYTE\b/u8/gm' src/*/*.rs
    perl -i -p0e 's/\b(?:libc::)?size_t\b/usize/gm' src/*/*.rs

    # Remove old typedefs
    perl -i -p0e 's/\npub type (u8|u16|u32|u64|usize|i8|i16|i32|i64|isize) = [^;]+;//gm' src/*/*.rs
    perl -i -p0e 's/\npub type (?:libc::)?size_t = [^;]+;//gm' src/*/*.rs

    ;;
  
  cast-sizeof)
    # Fix casting size_of from usize to c_ulong
    perl -i -p0e 's/(::core::mem::size_of::<[^>]*>\(\)) as [^;,]*?c_ulong\b/$1/gm' src/*/*.rs

    ;;

  cast-var-decl)
    # Remove unnecessary casts on variable declarations
    perl -i -p0e 's/(let (?:mut )[\w\d_]+)( = \d+)(?: as [\w\d:_]+)* as ([\w\d:_]+)/$1: $3$2/gm' src/*/*.rs

    ;;

  cast-neg1-max)
    # -1 as uint => uint::MAX
    # (-(1 as std::ffi::c_int) as u32)
    perl -i -p0e 's/-\(1(?: as [\w\d:_]+)*\) as ([\w\d:_]*(?:uint\d+|u\d+))/$1::MAX/gm' src/*/*.rs

    ;;

  cast-shift-rhs)
    # (c >> 8 as std::ffi::c_int)
    perl -i -p0e 's/((?:>>|<<|>>=|<<=) \d+)(?: as [\w\d:_]+)+/$1/gm' src/*/*.rs

    ;;

  cast-argument)
    # .offset((c >> 16) as u8 as isize), .wrapping_add(1 as std::ffi::c_int as u64)
    perl -i -p0e 's/(\.(?:wrapping_\w+|offset)\(\d+)(?: as [\w\d:_]*?)+\)/$1)/gm' src/*/*.rs

    ;;

  cast-constant)
    # (0 as std::ffi::c_int as usize)
    perl -i -p0e 's/\((\d+)(?: as [\w\d:_]+)* as ([ui]size|[ui]\d+)\)/$1_$2/gm' src/*/*.rs

    ;;

  cast-constant-c)
    # (31 as std::ffi::c_int as std::ffi::c_uint)
    perl -i -p0e 's/\((\d+)(?: as [\w\d:_]+)* as ([\w\d:_]+)\)/($1 as $2)/gm' src/*/*.rs

    ;;

  cast-rhs)
    # bitCount += 2 as std::ffi::c_int; , bitCount = 4 as std::ffi::c_int; , bitCount &= 31 as std::ffi::c_int;
    perl -i -p0e 's/((?: =|\+=|&=|\|=|\^=|-=|\*=|\/=) \d+)(?: as [\w\d:_]+)+;/$1;/gm' src/*/*.rs

    ;;

  cast-constant-neg)
    # .offset(-(1 as std::ffi::c_int) as isize)
    perl -i -p0e 's/(\.(?:wrapping_\w+|offset)\()-\((\d+)(?: as [\w\d:_]+)*\)(?: as [\w\d:_]*?)+\)/$1-$2)/gm' src/*/*.rs

    ;;

  cast-compare)
    # >= 128 as std::ffi::c_int as usize
    perl -i -p0e 's/((?:>=|<=|==|!=|>|<) \d+)(?: as [\w\d:_]+)+/$1/gm' src/*/*.rs

    ;;

  cast-terminated)
    # return 0 as std::ffi::c_int; 473195 as std::ffi::c_int as u32,
    perl -i -p0e 's/\b(\d+)(?: as [\w\d:_]+)+([,;])/$1$2/gm' src/*/*.rs

    ;;

  cast-multi)
    # 0 as std::ffi::c_int as std::ffi::c_uint
    perl -i -p0e 's/\b(\d+)(?: as [\w\d:_]+)+( as [\w\d:_]+)/$1$2/gm' src/*/*.rs

    ;;

  cast-term-paren)
    # 0 as std::ffi::c_int as usize)
    perl -i -p0e 's/(, *\d+)(?: as [\w\d:_]+)+\)/$1)/gm' src/*/*.rs

    ;;

  cast-index)
    # [0 as usize]
    perl -i -p0e 's/\[(\d+)(?: as [\w\d:_]+)+\]/[$1]/gm' src/*/*.rs

    ;;

  missing-imports)
    # Fix missing imports
    sed -i "2 s/use ::c2rust_bitfields;/use ::c2rust_bitfields::BitfieldStruct;/" src/compress/zstdmt_compress.rs

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

  casts)
    # Remove redundant as casts
    perl -i -p0e 's/\b(as \w\S*) \1\b//gm' src/*/*.rs
    perl -i -p0e 's/(.wrapping_(?:sub|add|div|mul|rem)\(\d+)(?: as \w\S*?)+\)/$1)/gm' src/*/*.rs

    # Remove unnecessary casts where type can be inferred
    perl -i -p0e 's/( (?:>=|<=|>|<|==|!=|&|&=|\*|\+|\|) \d+)( as [^\s\);,]+)+/$1/gm' src/*/*.rs
    perl -i -p0e 's/\((\d+)(?: as [^\s\);,]+?)+\)/($1)/gm' src/*/*.rs

    ;;

  *)
    echo "Unknown stage `$1`. Available stages:"
    echo "  transpile, missing-imports, asserts, integers"
    exit 1

    ;;

esac