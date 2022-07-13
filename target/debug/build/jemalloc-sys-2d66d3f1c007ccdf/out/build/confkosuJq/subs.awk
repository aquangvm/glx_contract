BEGIN {
S["LTLIBOBJS"]=""
S["LIBOBJS"]=""
S["cfgoutputs_out"]="Makefile jemalloc.pc doc/html.xsl doc/manpages.xsl doc/jemalloc.xml include/jemalloc/jemalloc_macros.h include/jemalloc/jemalloc_protos.h include/je"\
"malloc/jemalloc_typedefs.h include/jemalloc/internal/jemalloc_preamble.h test/test.sh test/include/test/jemalloc_test.h"
S["cfgoutputs_in"]="Makefile.in jemalloc.pc.in doc/html.xsl.in doc/manpages.xsl.in doc/jemalloc.xml.in include/jemalloc/jemalloc_macros.h.in include/jemalloc/jemalloc_p"\
"rotos.h.in include/jemalloc/jemalloc_typedefs.h.in include/jemalloc/internal/jemalloc_preamble.h.in test/test.sh.in test/include/test/jemalloc_test."\
"h.in"
S["cfghdrs_out"]="include/jemalloc/jemalloc_defs.h include/jemalloc/jemalloc.h include/jemalloc/internal/private_symbols.awk include/jemalloc/internal/private_symbols"\
"_jet.awk include/jemalloc/internal/public_symbols.txt include/jemalloc/internal/public_namespace.h include/jemalloc/internal/public_unnamespace.h in"\
"clude/jemalloc/internal/size_classes.h include/jemalloc/jemalloc_protos_jet.h include/jemalloc/jemalloc_rename.h include/jemalloc/jemalloc_mangle.h "\
"include/jemalloc/jemalloc_mangle_jet.h include/jemalloc/internal/jemalloc_internal_defs.h test/include/test/jemalloc_test_defs.h"
S["cfghdrs_in"]="include/jemalloc/jemalloc_defs.h.in include/jemalloc/internal/jemalloc_internal_defs.h.in include/jemalloc/internal/private_symbols.sh include/jemal"\
"loc/internal/private_namespace.sh include/jemalloc/internal/public_namespace.sh include/jemalloc/internal/public_unnamespace.sh include/jemalloc/int"\
"ernal/size_classes.sh include/jemalloc/jemalloc_rename.sh include/jemalloc/jemalloc_mangle.sh include/jemalloc/jemalloc.sh test/include/test/jemallo"\
"c_test_defs.h.in"
S["enable_initial_exec_tls"]="1"
S["enable_zone_allocator"]=""
S["enable_tls"]="0"
S["enable_lazy_lock"]="0"
S["jemalloc_version_gid"]="0000000000000000000000000000000000000000"
S["jemalloc_version_nrev"]="0"
S["jemalloc_version_bugfix"]="0"
S["jemalloc_version_minor"]="0"
S["jemalloc_version_major"]="0"
S["jemalloc_version"]="0.0.0-0-g0000000000000000000000000000000000000000"
S["enable_log"]="0"
S["enable_cache_oblivious"]="1"
S["enable_xmalloc"]="0"
S["enable_utrace"]="0"
S["enable_fill"]="1"
S["enable_prof"]="0"
S["enable_stats"]="1"
S["enable_debug"]="0"
S["je_"]="je_"
S["install_suffix"]=""
S["private_namespace"]="_rjem_je_"
S["JEMALLOC_CPREFIX"]="_RJEM_"
S["JEMALLOC_PREFIX"]="_rjem_"
S["AUTOCONF"]="false"
S["LD"]="/c/msys64/mingw64/bin/ld"
S["RANLIB"]="ranlib"
S["INSTALL_DATA"]="${INSTALL} -m 644"
S["INSTALL_SCRIPT"]="${INSTALL}"
S["INSTALL_PROGRAM"]="${INSTALL}"
S["enable_autogen"]="0"
S["RPATH_EXTRA"]=""
S["LM"]=""
S["CC_MM"]="1"
S["DUMP_SYMS"]="nm -a"
S["AROUT"]=" $@"
S["ARFLAGS"]="crus"
S["MKLIB"]=""
S["TEST_LD_MODE"]=""
S["LDTARGET"]="-o $@"
S["CTARGET"]="-o $@"
S["PIC_CFLAGS"]=""
S["SOREV"]="dll"
S["EXTRA_LDFLAGS"]=""
S["DSO_LDFLAGS"]="-shared"
S["link_whole_archive"]="1"
S["libprefix"]=""
S["exe"]=".exe"
S["a"]="lib"
S["o"]="o"
S["importlib"]="dll"
S["so"]="dll"
S["LD_PRELOAD_VAR"]="LD_PRELOAD"
S["RPATH"]=""
S["abi"]="pecoff"
S["AWK"]="gawk"
S["NM"]="nm"
S["AR"]="ar"
S["host_os"]="mingw32"
S["host_vendor"]="w64"
S["host_cpu"]="x86_64"
S["host"]="x86_64-w64-mingw32"
S["build_os"]="mingw32"
S["build_vendor"]="w64"
S["build_cpu"]="x86_64"
S["build"]="x86_64-w64-mingw32"
S["EGREP"]="/usr/bin/grep -E"
S["GREP"]="/usr/bin/grep"
S["EXTRA_CXXFLAGS"]=""
S["SPECIFIED_CXXFLAGS"]=""
S["CONFIGURE_CXXFLAGS"]=""
S["enable_cxx"]="0"
S["HAVE_CXX14"]=""
S["ac_ct_CXX"]=""
S["CXXFLAGS"]=""
S["CXX"]=""
S["CPP"]="gcc.exe -E"
S["EXTRA_CFLAGS"]=""
S["SPECIFIED_CFLAGS"]="-O0 -ffunction-sections -fdata-sections -g -fno-omit-frame-pointer -m64 -Wall"
S["CONFIGURE_CFLAGS"]="-std=gnu11 -Wall -Wsign-compare -Wundef -Wno-format-zero-length -pipe -g3 -O3 -funroll-loops"
S["OBJEXT"]="o"
S["EXEEXT"]=".exe"
S["ac_ct_CC"]=""
S["CPPFLAGS"]="-O0 -ffunction-sections -fdata-sections -g -fno-omit-frame-pointer -m64 -Wall -D_REENTRANT"
S["LDFLAGS"]="-O0 -ffunction-sections -fdata-sections -g -fno-omit-frame-pointer -m64 -Wall"
S["CFLAGS"]="-std=gnu11 -Wall -Wsign-compare -Wundef -Wno-format-zero-length -pipe -g3 -O3 -funroll-loops -O0 -ffunction-sections -fdata-sections -g -fno-omit-fr"\
"ame-pointer -m64 -Wall"
S["CC"]="gcc.exe"
S["XSLROOT"]=""
S["XSLTPROC"]="false"
S["MANDIR"]="d:\\Work\\dinar\\glx_contract\\target\\debug\\build\\jemalloc-sys-2d66d3f1c007ccdf\\out/share/man"
S["DATADIR"]="d:\\Work\\dinar\\glx_contract\\target\\debug\\build\\jemalloc-sys-2d66d3f1c007ccdf\\out/share"
S["LIBDIR"]="d:Workdinarglx_contracttargetdebugbuildjemalloc-sys-2d66d3f1c007ccdfout/lib"
S["INCLUDEDIR"]="d:Workdinarglx_contracttargetdebugbuildjemalloc-sys-2d66d3f1c007ccdfout/include"
S["BINDIR"]="d:Workdinarglx_contracttargetdebugbuildjemalloc-sys-2d66d3f1c007ccdfout/bin"
S["PREFIX"]="d:\\Work\\dinar\\glx_contract\\target\\debug\\build\\jemalloc-sys-2d66d3f1c007ccdf\\out"
S["abs_objroot"]="/d/Work/dinar/glx_contract/target/debug/build/jemalloc-sys-2d66d3f1c007ccdf/out/build/"
S["objroot"]=""
S["abs_srcroot"]="/d/Work/dinar/glx_contract/target/debug/build/jemalloc-sys-2d66d3f1c007ccdf/out/jemalloc/"
S["srcroot"]="d:/Work/dinar/glx_contract/target/debug/build/jemalloc-sys-2d66d3f1c007ccdf/out/jemalloc/"
S["rev"]="2"
S["CONFIG"]="--disable-cxx --with-jemalloc-prefix=_rjem_ --with-private-namespace=_rjem_ --host=x86_64-w64-mingw32 --build=x86_64-w64-mingw32 --prefix=d:\\Work\\di"\
"nar\\glx_contract\\target\\debug\\build\\jemalloc-sys-2d66d3f1c007ccdf\\out build_alias=x86_64-w64-mingw32 host_alias=x86_64-w64-mingw32 CC=gcc.exe 'CFLAG"\
"S=-O0 -ffunction-sections -fdata-sections -g -fno-omit-frame-pointer -m64 -Wall' 'LDFLAGS=-O0 -ffunction-sections -fdata-sections -g -fno-omit-frame"\
"-pointer -m64 -Wall' 'CPPFLAGS=-O0 -ffunction-sections -fdata-sections -g -fno-omit-frame-pointer -m64 -Wall'"
S["target_alias"]=""
S["host_alias"]="x86_64-w64-mingw32"
S["build_alias"]="x86_64-w64-mingw32"
S["LIBS"]=""
S["ECHO_T"]=""
S["ECHO_N"]="-n"
S["ECHO_C"]=""
S["DEFS"]="-DHAVE_CONFIG_H"
S["mandir"]="${datarootdir}/man"
S["localedir"]="${datarootdir}/locale"
S["libdir"]="${exec_prefix}/lib"
S["psdir"]="${docdir}"
S["pdfdir"]="${docdir}"
S["dvidir"]="${docdir}"
S["htmldir"]="${docdir}"
S["infodir"]="${datarootdir}/info"
S["docdir"]="${datarootdir}/doc/${PACKAGE}"
S["oldincludedir"]="/usr/include"
S["includedir"]="${prefix}/include"
S["localstatedir"]="${prefix}/var"
S["sharedstatedir"]="${prefix}/com"
S["sysconfdir"]="${prefix}/etc"
S["datadir"]="${datarootdir}"
S["datarootdir"]="${prefix}/share"
S["libexecdir"]="${exec_prefix}/libexec"
S["sbindir"]="${exec_prefix}/sbin"
S["bindir"]="${exec_prefix}/bin"
S["program_transform_name"]="s,x,x,"
S["prefix"]="d:\\Work\\dinar\\glx_contract\\target\\debug\\build\\jemalloc-sys-2d66d3f1c007ccdf\\out"
S["exec_prefix"]="d:\\Work\\dinar\\glx_contract\\target\\debug\\build\\jemalloc-sys-2d66d3f1c007ccdf\\out"
S["PACKAGE_URL"]=""
S["PACKAGE_BUGREPORT"]=""
S["PACKAGE_STRING"]=""
S["PACKAGE_VERSION"]=""
S["PACKAGE_TARNAME"]=""
S["PACKAGE_NAME"]=""
S["PATH_SEPARATOR"]=":"
S["SHELL"]="/bin/sh"
  for (key in S) S_is_set[key] = 1
  FS = ""

}
{
  line = $ 0
  nfields = split(line, field, "@")
  substed = 0
  len = length(field[1])
  for (i = 2; i < nfields; i++) {
    key = field[i]
    keylen = length(key)
    if (S_is_set[key]) {
      value = S[key]
      line = substr(line, 1, len) "" value "" substr(line, len + keylen + 3)
      len += length(value) + length(field[++i])
      substed = 1
    } else
      len += 1 + keylen
  }

  print line
}

