savedcmd_/home/acetim/rpi-oled-driver/src/lib.o := RUST_MODFILE=/home/acetim/rpi-oled-driver/oled_i2c_bitbang rustc --edition=2021 -Zbinary_dep_depinfo=y -Astable_features -Aunused_features -Dnon_ascii_idents -Dunsafe_op_in_unsafe_fn -Wmissing_docs -Wrust_2018_idioms -Wunreachable_pub -Wclippy::all -Wclippy::ignored_unit_patterns -Wclippy::mut_mut -Wclippy::needless_bitwise_bool -Aclippy::needless_lifetimes -Wclippy::no_mangle_with_rust_abi -Wclippy::undocumented_unsafe_blocks -Wclippy::unnecessary_safety_comment -Wclippy::unnecessary_safety_doc -Wrustdoc::missing_crate_level_docs -Wrustdoc::unescaped_backticks -Cpanic=abort -Cembed-bitcode=n -Clto=n -Cforce-unwind-tables=n -Ccodegen-units=1 -Csymbol-mangling-version=v0 -Crelocation-model=static -Zfunction-sections=n -Wclippy::float_arithmetic --target=aarch64-unknown-none-softfloat -Cforce-unwind-tables=n -Zbranch-protection=bti,pac-ret -Copt-level=2 -Cdebug-assertions=n -Coverflow-checks=y -Cforce-frame-pointers=y  --cfg MODULE  @./include/generated/rustc_cfg -Zallow-features=arbitrary_self_types,lint_reasons,used_with_arg -Zcrate-attr=no_std -Zcrate-attr='feature(arbitrary_self_types,lint_reasons,used_with_arg)' -Zunstable-options --extern kernel --crate-type rlib -L ./rust/ --crate-name lib --sysroot=/dev/null --out-dir /home/acetim/rpi-oled-driver/src/ --emit=dep-info=/home/acetim/rpi-oled-driver/src/.lib.o.d --emit=obj=/home/acetim/rpi-oled-driver/src/lib.o /home/acetim/rpi-oled-driver/src/lib.rs

source_/home/acetim/rpi-oled-driver/src/lib.o := /home/acetim/rpi-oled-driver/src/lib.rs

deps_/home/acetim/rpi-oled-driver/src/lib.o := \
  /home/acetim/rpi-oled-driver/src/i2c_basic_components.rs \
  /home/acetim/rpi-oled-driver/src/i2c_oled_handler.rs \
  ./rust/libcore.rmeta \
  ./rust/libkernel.rmeta \
  ./rust/libffi.rmeta \
  ./rust/libcompiler_builtins.rmeta \
  ./rust/libmacros.so \
  ./rust/libbindings.rmeta \
  ./rust/libuapi.rmeta \
  ./rust/libbuild_error.rmeta \

/home/acetim/rpi-oled-driver/src/lib.o: $(deps_/home/acetim/rpi-oled-driver/src/lib.o)

$(deps_/home/acetim/rpi-oled-driver/src/lib.o):
