# For Command Line Tools, use this:
#export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/usr/lib/"

# For XCode, use this:
export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/Toolchains/XcodeDefault.xctoolchain/usr/lib/"
