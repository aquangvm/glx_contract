#!/bin/sh

prefix=d:\Work\dinar\glx_contract\target\debug\build\jemalloc-sys-2d66d3f1c007ccdf\out
exec_prefix=d:\Work\dinar\glx_contract\target\debug\build\jemalloc-sys-2d66d3f1c007ccdf\out
libdir=${exec_prefix}/lib

LD_PRELOAD=${libdir}/libjemalloc.dll
export LD_PRELOAD
exec "$@"
