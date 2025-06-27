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
    c2rust transpile --emit-no-std --emit-build-files --overwrite-existing --reduce-type-annotations --translate-const-macros --translate-fn-macros \
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
    perl -i -p0e 's/\[(\d+)(?:[\s\n]* as [\w\d:_]+)+\]/[$1]/gm' src/*/*.rs

    ;;

  cast-neg-term)
    # -(1 as std::ffi::c_int) as isize,
    perl -i -p0e 's/-\((\d+)(?: as [\w\d:_]+)*\)(?: as [\w\d:_]+)+([,;])/-$1$2/gm' src/*/*.rs

    ;;

  cast-primitive)
    # 0 as usize
    perl -i -p0e 's/\b(\d+) as ([ui]size|[ui]\d+)/$1_$2/gm' src/*/*.rs

    ;;

  error)
    # ERROR!(maxCode)
    perl -i -p0e 's/ERROR!\((\w+)\)/ERROR(ZSTD_error_$1)/gm'  src/*/*.rs

    ;;

  min-max)
    # MIN!(ofBits, STREAM_ACCUMULATOR_MIN - 1)
    # MAX!(minIndexToOverflowCorrect * adjustment, minIndexToOverflowCorrect)
    perl -i -p0e 's/\bMIN!\(/std::cmp::min(/gm'  src/*/*.rs
    perl -i -p0e 's/\bMAX!\(/std::cmp::max(/gm'  src/*/*.rs

    # std::cmp::max(minTarget, cctxParams -> targetCBlockSize)
    # std::cmp::max(
    #     21, ZSTD_cycleLog(params -> cParams.chainLog, params -> cParams.strategy) + 3
    # )
    # jobLog = std::cmp::max(20, params -> cParams.windowLog + 2);
    # std::cmp::max(
    #     (usize) ZSTD_FRAMEHEADERSIZE_MIN(zds -> format), hSize
    # )
    perl -i -p0e 's/(std::cmp::(?:min|max)\(\n?.*?)\b([\w_\d]+) -> (.*?)\b([\w_\d]+) -> /$1(*$2).$3(*$4)./gm'  src/*/*.rs
    perl -i -p0e 's/(std::cmp::(?:min|max)\(\n?.*?)\b([\w_\d]+) -> /$1(*$2)./gm'  src/*/*.rs

    # std::cmp::max(elt.length - table[u].length, 1)
    # std::cmp::max(
    #     info.maxNbAdditionalBits, table[u].nbAdditionalBits
    # )
    perl -i -p0e 's/(std::cmp::(?:min|max)\(\n?.*?)\b([\w_\d]+)\[([\w_\d]+)\]/$1(*$2.offset($3 as isize))/gm'  src/*/*.rs

    ;;

  libc-alloc-mem)
    # fn malloc(_: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    # fn calloc(_: std::ffi::c_ulong, _: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    # fn memset(
    #     _: *mut std::ffi::c_void,
    #     _: std::ffi::c_int,
    #     _: std::ffi::c_ulong,
    # ) -> *mut std::ffi::c_void;
    perl -i -p0e 's/\n\s*fn (?:malloc|calloc|free|memset|memcpy|memmove)\([^;]*?;//gm' src/*/*.rs

    # newBuff = malloc(sBuffSize.wrapping_add(NOISELENGTH as usize));
    # memcpy(newBuff, samplesBuffer, sBuffSize);
    # memset(
    perl -i -p0e 's/([^:\w_\d])(malloc|calloc|free|memset|memcpy|memmove)\(/$1libc::$2(/gm' src/*/*.rs

    ;;

  splitpoint-float)
    # parameters.splitPoint <= 0
    # parameters.splitPoint > 1
    perl -i -p0e 's/(splitPoint [><=]+ \d+)(^\.)/$1.0$2/gm' src/*/*.rs

    ;;

  displaylevel)
    # if DISPLAYLEVEL!(1, "FASTCOVER parameters incorrect\n") >= 1 {
    #     fprintf(
    #         stderr,
    #         b"FASTCOVER parameters incorrect\n\0" as *const u8
    #             as *const std::ffi::c_char,
    #     );
    #     fflush(stderr);
    # }
    # if DISPLAYLEVEL!(
    #     1, "dictBufferCapacity must be at least %u\n", ZDICT_DICTSIZE_MIN
    # ) >= 1
    # {
    #     fprintf(
    #         stderr,
    #         b"dictBufferCapacity must be at least %u\n\0" as *const u8
    #             as *const std::ffi::c_char,
    #         256,
    #     );
    #     fflush(stderr);
    # }
    perl -i -p0e 's/if DISPLAYLEVEL!\([\s\n]*([^,]+),[\s\n]*((?:"(?:\\"|[^"])+"[\s\n*]*?)+)(?:[\s\n]*(,[\s\n]*(?:\([^\)]+\)|[^,\)]+?)+?)[\s\n]*)?(?:[\s\n]*(,[\s\n]*(?:\([^\)]+\)|[^,\)]+?)+?)[\s\n]*)?(?:[\s\n]*(,[\s\n]*(?:\([^\)]+\)|[^,\)]+?)+?)[\s\n]*)?(?:[\s\n]*(,[\s\n]*(?:\([^\)]+\)|[^,\)]+?)+?)[\s\n]*)?[\s\n]*\)[^\{]+\{[^\}]+\}/display_level!($1, $2$3$4$5$6);/gm' src/*/*.rs

    ;;

  zstd-alloc-mem)
    # ZSTD_calloc!(1, size)
    # ZSTD_malloc!(size)
    perl -i -p0e 's/\bZSTD_(malloc|calloc)!\(/libc::$1(/gm' src/*/*.rs

    # ZSTD_ALIGNOF!(u32)
    perl -i -p0e 's/\bZSTD_ALIGNOF!\(([^\)]+)\)/std::mem::align_of::<$1>()/gm' src/*/*.rs

    # ZSTD_free!(ptr)(ZSTD_free!(ptr))
    perl -i -p0e 's/\bZSTD_free!(ptr)(ZSTD_free!(ptr))/libc::free(ptr)/gm' src/*/*.rs

    # libc::memset(
    #     ZSTD_memset!(ptr, 0, size),
    #     ZSTD_memset!(ptr, 0, size),
    #     ZSTD_memset!(ptr, 0, size) as usize,
    # );
    # libc::memset(
    #     ZSTD_memset!(rankPosition, 0, sizeof(* rankPosition) * RANK_POSITION_TABLE_SIZE),
    #     ZSTD_memset!(rankPosition, 0, sizeof(* rankPosition) * RANK_POSITION_TABLE_SIZE),
    #     ZSTD_memset!(rankPosition, 0, sizeof(* rankPosition) * RANK_POSITION_TABLE_SIZE)
    #         as usize,
    # );
    # libc::memset(
    #     ZSTD_memset!(ms -> tagTable, 0, tagTableSize),
    #     ZSTD_memset!(ms -> tagTable, 0, tagTableSize),
    #     ZSTD_memset!(ms -> tagTable, 0, tagTableSize) as usize,
    # );
    # libc::memset(
    #     ZSTD_memset!(dst, * (const u8 *) cSrc, dstSize),
    #     ZSTD_memset!(dst, * (const u8 *) cSrc, dstSize),
    #     ZSTD_memset!(dst, * (const u8 *) cSrc, dstSize) as usize,
    # );
    # libc::memset(
    #     ZSTD_memset!(
    #         dctx -> litBuffer, istart[lhSize], litSize -
    #         ZSTD_LITBUFFEREXTRASIZE
    #     ),
    #     ZSTD_memset!(
    #         dctx -> litBuffer, istart[lhSize], litSize -
    #         ZSTD_LITBUFFEREXTRASIZE
    #     ),
    #     ZSTD_memset!(
    #         dctx -> litBuffer, istart[lhSize], litSize -
    #         ZSTD_LITBUFFEREXTRASIZE
    #     ) as usize,
    # );
    # libc::memcpy(
    #     ZSTD_memcpy!(buffer, headerBuffer, hbSize),
    #     ZSTD_memcpy!(buffer, headerBuffer, hbSize),
    #     ZSTD_memcpy!(buffer, headerBuffer, hbSize) as usize,
    # );
    # libc::memcpy(
    #     ZSTD_memcpy!((u8 *) dst + ZSTD_blockHeaderSize, src, srcSize),
    #     ZSTD_memcpy!((u8 *) dst + ZSTD_blockHeaderSize, src, srcSize),
    #     ZSTD_memcpy!((u8 *) dst + ZSTD_blockHeaderSize, src, srcSize) as usize,
    # );
    # libc::memcpy(
    #     ZSTD_memcpy!(op, hufMetadata -> hufDesBuffer, hufMetadata -> hufDesSize),
    #     ZSTD_memcpy!(op, hufMetadata -> hufDesBuffer, hufMetadata -> hufDesSize),
    #     ZSTD_memcpy!(op, hufMetadata -> hufDesBuffer, hufMetadata -> hufDesSize)
    #         as usize,
    # );
    perl -i -p0e 's/\blibc::(memset|memcpy|memmove)\([\s\n]*ZSTD_(?:memset|memcpy|memmove)!\([\s\n]*([^,]+),[\s\n]*([^,]+),[\s\n]*((?:\([^\)]+\)|[^\)])+?)[\s\n]*\)[^;]+;/libc::$1($2, $3, ($4) as usize);/gm' src/*/*.rs

    # fix -> in memset and memcpy
    perl -i -p0e 's/(libc::(?:memcpy|memset|memmove)\(\n?.*?)\b([\w_\d]+) -> (.*?)\b([\w_\d]+) -> /$1(*$2).$3(*$4)./gm'  src/*/*.rs
    perl -i -p0e 's/(libc::(?:memcpy|memset|memmove)\(\n?.*?)\b([\w_\d]+) -> /$1(*$2)./gm'  src/*/*.rs

    # fix x[y] in memset and memcpy
    perl -i -p0e 's/(libc::(?:memcpy|memset|memmove)\(\n?.*?)\b([\w_\d]+)\[([\w_\d]+)\]/$1(*$2.offset($3 as isize))/gm'  src/*/*.rs

    ;;

  likely-unlikely)
    # fix -> in LIKELY and UNLIKELY
    perl -i -p0e 's/\b((?:UN)?LIKELY!\(\n?.*?)\b([\w_\d]+) -> (.*?)\b([\w_\d]+) -> /$1(*$2).$3(*$4)./gm'  src/*/*.rs
    perl -i -p0e 's/\b((?:UN)?LIKELY!\(\n?.*?)\b([\w_\d]+) -> /$1(*$2)./gm'  src/*/*.rs

    # fix x[y] in LIKELY and UNLIKELY
    perl -i -p0e 's/\b((?:UN)?LIKELY!\(\n?.*?)\b([\w_\d]+)\[([\w_\d]+)\]/$1(*$2.offset($3 as isize))/gm'  src/*/*.rs

    ;;

  copy8-16)
    # COPY8!(op, ip)(op as *mut std::ffi::c_void, ip as *const std::ffi::c_void);
    # op = op.offset(COPY8!(op, ip) as isize);
    # ip = ip.offset(COPY8!(op, ip) as isize);
    # COPY16!(op, ip)(op as *mut std::ffi::c_void, ip as *const std::ffi::c_void);
    # op = op.offset(COPY16!(op, ip) as isize);
    # ip = ip.offset(COPY16!(op, ip) as isize);
    # COPY16!(op, ip)(op as *mut std::ffi::c_void, ip as *const std::ffi::c_void);
    # op = op.offset(COPY16!(op, ip) as isize);
    # ip = ip.offset(COPY16!(op, ip) as isize);
    perl -i -p0e 's/\bCOPY(8|16)!\(op, ip\)\([^;]*;[^;]*;[^;]*;/COPY$1!(op, ip);/gm'  src/*/*.rs

    ;;

  swap)
    # let ref mut fresh132 = SWAP!(v1, v4);
    # *fresh132 = SWAP!(v1, v4);
    # let ref mut fresh106 = SWAP!(SA[m], SA[m / 2]);
    # *fresh106 = SWAP!(SA[m], SA[m / 2]);
    perl -i -p0e 's/let ref mut \w[\w\d_]* = (SWAP!\([^\)]+\);)[^;]*;/$1/gm'  src/*/*.rs

    # fix x[y] in SWAP
    perl -i -p0e 's/\b(SWAP!\(\n?.*?)\b([\w_\d]+)\[([^\]]+)\]/$1(*$2.offset($3 as isize))/gm'  src/*/*.rs
    perl -i -p0e 's/\b(SWAP!\(\n?.*?)\b([\w_\d]+)\[([^\]]+)\]/$1(*$2.offset($3 as isize))/gm'  src/*/*.rs

    ;;

  forward-if-error)
    # let err_code = FORWARD_IF_ERROR!(ZSTD_checkCParams(params.cParams), "");
    # if FORWARD_IF_ERROR!(ZSTD_checkCParams(params.cParams), "") != 0 {
    #     return FORWARD_IF_ERROR!(ZSTD_checkCParams(params.cParams), "");
    # }

    # let err_code = FORWARD_IF_ERROR!(
    #     FSE_normalizeCount(norm, tableLog, count, nbSeq, max,
    #     ZSTD_useLowProbCount(nbSeq)), ""
    # );
    # if FORWARD_IF_ERROR!(
    #     FSE_normalizeCount(norm, tableLog, count, nbSeq, max,
    #     ZSTD_useLowProbCount(nbSeq)), ""
    # ) != 0
    # {
    #     return FORWARD_IF_ERROR!(
    #         FSE_normalizeCount(norm, tableLog, count, nbSeq, max,
    #         ZSTD_useLowProbCount(nbSeq)), ""
    #     );
    # }
    perl -i -p0e 's/let \w[\w\d_]* = (FORWARD_IF_ERROR!\([^;]*\);)[^;]*;[\s\n]*}/$1/gm'  src/*/*.rs

    # fix ->
    perl -i -p0e 's/\b(FORWARD_IF_ERROR!\([^;]*?[ \(])((?:\([^\(\)&,]*\)|[^->\(\)&,])+) ->[\s\n]+/$1(*$2)./gm'  src/*/*.rs
    perl -i -p0e 's/\b(FORWARD_IF_ERROR!\([^;]*?[ \(])((?:\([^\(\)&,]*\)|[^->\(\)&,])+) ->[\s\n]+/$1(*$2)./gm'  src/*/*.rs
    perl -i -p0e 's/\b(FORWARD_IF_ERROR!\([^;]*?[ \(])((?:\([^\(\)&,]*\)|[^->\(\)&,])+) ->[\s\n]+/$1(*$2)./gm'  src/*/*.rs
    perl -i -p0e 's/\b(FORWARD_IF_ERROR!\([^;]*?[ \(])((?:\([^\(\)&,]*\)|[^->\(\)&,])+) ->[\s\n]+/$1(*$2)./gm'  src/*/*.rs
    perl -i -p0e 's/\b(FORWARD_IF_ERROR!\([^;]*?[ \(])((?:\([^\(\)&,]*\)|[^->\(\)&,])+) ->[\s\n]+/$1(*$2)./gm'  src/*/*.rs
    perl -i -p0e 's/\b(FORWARD_IF_ERROR!\([^;]*?[ \(])((?:\([^\(\)&,]*\)|[^->\(\)&,])+) ->[\s\n]+/$1(*$2)./gm'  src/*/*.rs
    perl -i -p0e 's/\b(FORWARD_IF_ERROR!\([^;]*?[ \(])((?:\([^\(\)&,]*\)|[^->\(\)&,])+) ->[\s\n]+/$1(*$2)./gm'  src/*/*.rs
    perl -i -p0e 's/\b(FORWARD_IF_ERROR!\([^;]*?[ \(])((?:\([^\(\)&,]*\)|[^->\(\)&,])+) ->[\s\n]+/$1(*$2)./gm'  src/*/*.rs
    perl -i -p0e 's/\b(FORWARD_IF_ERROR!\([^;]*?[ \(])((?:\([^\(\)&,]*\)|[^->\(\)&,])+) ->[\s\n]+/$1(*$2)./gm'  src/*/*.rs
    perl -i -p0e 's/\b(FORWARD_IF_ERROR!\([^;]*?[ \(])((?:\([^\(\)&,]*\)|[^->\(\)&,])+) ->[\s\n]+/$1(*$2)./gm'  src/*/*.rs
    
    # fix (int) cast
    perl -i -p0e 's/\b(FORWARD_IF_ERROR!\([^;]*?)\((int|long)\) ([^,\)]+)/$1($3 as std::ffi::c_$2)/gm'  src/*/*.rs
    perl -i -p0e 's/\b(FORWARD_IF_ERROR!\([^;]*?)\((int|long)\) ([^,\)]+)/$1($3 as std::ffi::c_$2)/gm'  src/*/*.rs

    # fix addr-of &
    perl -i -p0e 's/\b(FORWARD_IF_ERROR!\([^;]*?)&[\s\n]+((?:\((?:\([^\)]*\)|[^\)])*\)|[^,\(\)])+)/$1addr_of!($2)/gm'  src/*/*.rs
    perl -i -p0e 's/\b(FORWARD_IF_ERROR!\([^;]*?)&[\s\n]+((?:\((?:\([^\)]*\)|[^\)])*\)|[^,\(\)])+)/$1addr_of!($2)/gm'  src/*/*.rs
    perl -i -p0e 's/\b(FORWARD_IF_ERROR!\([^;]*?)&[\s\n]+((?:\((?:\([^\)]*\)|[^\)])*\)|[^,\(\)])+)/$1addr_of!($2)/gm'  src/*/*.rs
    perl -i -p0e 's/\b(FORWARD_IF_ERROR!\([^;]*?)&[\s\n]+((?:\((?:\([^\)]*\)|[^\)])*\)|[^,\(\)])+)/$1addr_of!($2)/gm'  src/*/*.rs
    perl -i -p0e 's/\b(FORWARD_IF_ERROR!\([^;]*?)&[\s\n]+((?:\((?:\([^\)]*\)|[^\)])*\)|[^,\(\)])+)/$1addr_of!($2)/gm'  src/*/*.rs

    ;;

  stack-pop)
    # if STACK_POP!(first, last, depth, limit) != 0 {
    #     return;
    # }
    perl -i -p0e 's/\bif (STACK_POP5?!)\(([^\)]+)\) != 0 {[^\}]*}/$1(stack, $2);/gm'  src/*/*.rs

    ;;

  swap3)
    # SWAP!((*SA.offset(m as isize)), (*SA.offset(m / 2 as isize)));
    # SWAP!((*SA.offset(m as isize)), (*SA.offset(m / 2 as isize)));
    # SWAP!((*SA.offset(m as isize)), (*SA.offset(m / 2 as isize)));
    perl -i -p0e 's/\b(SWAP!\([^;]*\);)[\n\s]*\1[\n\s]*\1/$1/gm'  src/*/*.rs

    ;;

  stack-push)
    # if ssize < 64 {} else {
    #    __assert_fail(
    perl -i -p0e 's/\bif ssize < \d+ {} else {[^}]*__assert_fail[^}]*divsufsort\.c[^,]*,[\n\s]*(\d+),[^}]*}[\s\n]*.[\w\d_]*: {[\s\n]*if ssize < \d+ {} else {[^}]*__assert_fail[^}]*divsufsort\.c[^,]*,[\n\s]*\1,[^}]*}[^}]*};/STACK_PUSH!(???, #L$1);/gm'  src/*/*.rs

    perl -i -p0e 's/\bSTACK_PUSH!\(\?\?\?, #L(\d+)\);[\s\n]*stack\[ssize as usize\].a = ([^;]*);[\s\n]*stack\[ssize as usize\].b = ([^;]*);[\s\n]*stack\[ssize as usize\].c = ([^;]*);[^;]*;[^;]*;[\s\n]*stack\[[^\]]*\][\s\n]*.d = ([^;]*);/STACK_PUSH!($2, $3, $4, $5); \/\/ #L$1/gm'  src/*/*.rs
    
    ;;

  prefetch-area)
    perl -i -p0e 's/let [^=]*= (PREFETCH_AREA!\([^\)]*\);)[^;]*;[^;]*;[^;]*;[^;]*;[^;]*;[^;]*;[^}]*}/$1/gm'  src/*/*.rs

    ;;

  zstd-gen-fn)
    # ZSTD_GEN_DFAST_FN!
    perl -i -p0e 's/unsafe extern "C" fn (ZSTD_compressBlock_doubleFast_[^_]+)_(\d)\([^{]*{[^{]*ZSTD_GEN_DFAST_FN![^}]*}/ZSTD_GEN_DFAST_FN!($1_generic, $1_$2);/gm'  src/*/*.rs
    # ZSTD_GEN_FAST_FN!
    perl -i -p0e 's/unsafe extern "C" fn (ZSTD_compressBlock_fast_[^_]+)_(\d)_(\d)\([^{]*{[^{]*ZSTD_GEN_FAST_FN![^}]*}/ZSTD_GEN_FAST_FN!($1_generic, cmov=$3, $1_$2_$3);/gm'  src/*/*.rs

    ;;

  bounded)
    # fix -> in BOUNDED
    perl -i -p0e 's/\b(BOUNDED!\(\n?.*?)\b(\w+) -> /$1(*$2)./gm'  src/*/*.rs
    perl -i -p0e 's/\b(BOUNDED!\(\n?.*?)\b(\w+) -> /$1(*$2)./gm'  src/*/*.rs
    perl -i -p0e 's/\b(BOUNDED!\(\n?.*?)\b(\w+) -> /$1(*$2)./gm'  src/*/*.rs

    ;;

  huf-decode-symbol)
    # let ref mut fresh6 = HUF_DECODE_SYMBOLX1_0!(p, bitDPtr);
    # *fresh6 = HUF_DECODE_SYMBOLX1_0!(p, bitDPtr);
    perl -i -p0e 's/let [^=]*= (HUF_DECODE_SYMBOLX1_0!\([^)]*\);)[^;]*\1/$1/gm'  src/*/*.rs
    # if HUF_DECODE_SYMBOLX2_1!(op2, & bitD2) != 0 {
    #     op2 = op2.offset(HUF_DECODE_SYMBOLX2_1!(op2, & bitD2) as isize);
    # }
    # if HUF_DECODE_SYMBOLX1_2!(op1, & bitD1) != 0 {
    #     let fresh9 = op1;
    #     op1 = op1.offset(1);
    #     *fresh9 = HUF_decodeSymbolX1(&mut bitD1, dt, dtLog);
    # }
    perl -i -p0e 's/if (HUF_DECODE_SYMBOLX[12]_\d!\([^)]*\))[^}]*}/$1;/gm'  src/*/*.rs

    # fix addr-of &
    perl -i -p0e 's/\b(HUF_DECODE_SYMBOL\w+!\([^;]*?)&[\s\n]+((?:\((?:\([^\)]*\)|[^\)])*\)|[^,\(\)])+)/$1addr_of!($2)/gm'  src/*/*.rs

    ;;

  zstd-gen-record-fingerprint)
    # ZSTD_GEN_RECORD_FINGERPRINT!
    perl -i -p0e 's/unsafe extern "C" fn (ZSTD_recordFingerprint)_(\d+)\([^{]*{[^}]*ZSTD_GEN_RECORD_FINGERPRINT!\(\2, (\d+)\)[^}]*}/ZSTD_GEN_RECORD_FINGERPRINT!($1_$2, $3);/gm'  src/*/*.rs

    ;;

  weight)
    # fix -> in WEIGHT
    perl -i -p0e 's/\b(WEIGHT!\(\n?.*?)\b(\w+) -> /$1(*$2)./gm'  src/*/*.rs

    # fix x[y] in WEIGHT
    perl -i -p0e 's/\b(WEIGHT!\((?:[^,]+,)*(?:[^\[]*\[)?)((?:\([^\]();]*\)|[^\]();])+)\[([^\]\[;]+)\]/$1*$2.offset($3 as isize)/gm'  src/*/*.rs
    perl -i -p0e 's/\b(WEIGHT!\((?:[^,]+,)*(?:[^\[]*\[)?)((?:\([^\]();]*\)|[^\]();])+)\[([^\]\[;]+)\]/$1*$2.offset($3 as isize)/gm'  src/*/*.rs

    ;;

  bt-get-all-matches)
    # ZSTD_BT_GET_ALL_MATCHES_ARRAY!
    perl -i -p0e 's/\b(ZSTD_BT_GET_ALL_MATCHES_ARRAY!)\((\w+)\)/[ZSTD_btGetAllMatches_$2_3, ZSTD_btGetAllMatches_$2_4, ZSTD_btGetAllMatches_$2_5, ZSTD_btGetAllMatches_$2_6]/gm'  src/*/*.rs

    ;;

  fse-getsymbol)
    # & in FSE_GETSYMBOL!
    perl -i -p0e 's/\b(FSE_GETSYMBOL!\()& ?([^)]+)\)/$1addr_of!($2))/gm'  src/*/*.rs

    ;;

  fse-flushbits)
    # if FSE_FLUSHBITS!(& bitC) != 0 {} else {};
    perl -i -p0e 's/if (FSE_FLUSHBITS!\()& ?([^)]+)\)[^}]*}[^}]*}/$1addr_of!($2))/gm'  src/*/*.rs

    ;;

  boundcheck)
    # if ZSTD_cParam_withinBounds(ZSTD_c_searchLog, value) == 0 {
    #     return -(ZSTD_error_parameter_outOfBound as std::ffi::c_int)
    #         as usize;
    # }
    perl -i -p0e 's/if ZSTD_cParam_withinBounds\(([^\)]+)\)[^\}]*}/BOUNDCHECK!($1);/gm'  src/*/*.rs

    ;;

  reset)
    ./convert.sh clean
    ./convert.sh transpile
    ./convert.sh file-structure
    git restore src/lib.rs
    git restore Cargo.lock
    ;;

  redo)
    ./convert.sh missing-m128i_u
    ./convert.sh integers
    ./convert.sh cast-sizeof
    ./convert.sh cast-var-decl
    ./convert.sh cast-neg1-max
    ./convert.sh cast-shift-rhs
    ./convert.sh cast-argument
    ./convert.sh cast-constant
    ./convert.sh cast-constant-c
    ./convert.sh cast-rhs
    ./convert.sh cast-constant-neg
    ./convert.sh cast-compare
    ./convert.sh cast-terminated
    ./convert.sh cast-multi
    ./convert.sh cast-term-paren
    ./convert.sh cast-index
    ./convert.sh cast-neg-term
    ./convert.sh cast-primitive
    ./convert.sh error
    ./convert.sh min-max
    ./convert.sh libc-alloc-mem
    ./convert.sh splitpoint-float
    ./convert.sh zstd-alloc-mem
    ./convert.sh likely-unlikely
    ./convert.sh copy8-16
    ./convert.sh swap
    ./convert.sh forward-if-error
    ./convert.sh stack-pop
    ./convert.sh swap3
    ./convert.sh stack-push
    ./convert.sh prefetch-area
    ./convert.sh zstd-gen-fn
    ./convert.sh bounded
    ./convert.sh huf-decode-symbol
    ./convert.sh zstd-gen-record-fingerprint
    ./convert.sh weight
    ./convert.sh bt-get-all-matches
    ./convert.sh fse-getsymbol
    ./convert.sh fse-flushbits
    ./convert.sh boundcheck

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
    echo "Unknown stage `$1`."
    exit 1

    ;;

esac