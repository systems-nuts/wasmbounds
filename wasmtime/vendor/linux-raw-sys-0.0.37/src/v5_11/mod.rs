#[cfg(target_arch = "arm")]
mod arm;
#[cfg(target_arch = "arm")]
pub use arm::*;
#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use aarch64::*;
#[cfg(target_arch = "mips")]
mod mips;
#[cfg(target_arch = "mips")]
pub use mips::*;
#[cfg(target_arch = "mips64")]
mod mips64;
#[cfg(target_arch = "mips64")]
pub use mips64::*;
#[cfg(target_arch = "powerpc")]
mod powerpc;
#[cfg(target_arch = "powerpc")]
pub use powerpc::*;
#[cfg(target_arch = "powerpc64")]
mod powerpc64;
#[cfg(target_arch = "powerpc64")]
pub use powerpc64::*;
#[cfg(target_arch = "riscv32")]
mod riscv32;
#[cfg(target_arch = "riscv32")]
pub use riscv32::*;
#[cfg(target_arch = "riscv64")]
mod riscv64;
#[cfg(target_arch = "riscv64")]
pub use riscv64::*;
#[cfg(target_arch = "s390x")]
mod s390x;
#[cfg(target_arch = "s390x")]
pub use s390x::*;
#[cfg(target_arch = "sparc")]
mod sparc;
#[cfg(target_arch = "sparc")]
pub use sparc::*;
#[cfg(target_arch = "sparc64")]
mod sparc64;
#[cfg(target_arch = "sparc64")]
pub use sparc64::*;
#[cfg(target_arch = "x86")]
mod x86;
#[cfg(target_arch = "x86")]
pub use x86::*;
#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;
