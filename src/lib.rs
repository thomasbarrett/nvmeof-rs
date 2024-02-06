#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// bindgen /usr/src/linux-headers-$(uname -r)/include/linux/nvme.h \
// --constified-enum '*' \
// --with-derive-default \
// --no-layout-tests \
// --no-doc-comments \
// --allowlist-file=/usr/src/linux-headers-$(uname -r)/include/linux/nvme.h \
// -- -I/usr/src/linux-headers-$(uname -r)/include/ \
// -I/usr/src/linux-headers-$(uname -r)/arch/x86/include/generated \
// -I/usr/src/linux-headers-$(uname -r)/arch/x86/include \
// -I/usr/src/linux-headers-$(uname -r)/include/linux \
// -include /usr/src/linux-headers-$(uname -r)/include/linux/kconfig.h \
// -D__KERNEL__ \
// > nvme.rs
pub mod nvme;

// bindgen /usr/src/linux-headers-$(uname -r)/include/linux/nvme-tcp.h \
// --constified-enum '*' \
// --with-derive-default \
// --no-layout-tests \
// --no-doc-comments \
// --allowlist-file=/usr/src/linux-headers-$(uname -r)/include/linux/nvme-tcp.h \
// -- -I/usr/src/linux-headers-$(uname -r)/include/ \
// -I/usr/src/linux-headers-$(uname -r)/arch/x86/include/generated \
// -I/usr/src/linux-headers-$(uname -r)/arch/x86/include \
// -I/usr/src/linux-headers-$(uname -r)/include/linux \
// -include /usr/src/linux-headers-$(uname -r)/include/linux/kconfig.h \
// -D__KERNEL__ \
// > nvme_tcp.rs
pub mod nvme_tcp;
