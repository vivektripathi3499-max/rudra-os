# Rudra OS 🚧

Rudra OS is a custom **x86_64 operating system written in Rust**, currently under active development.

---

## 🔧 Current Features (Work in Progress)

* Memory management (paging, heap allocation)
* Interrupt handling (IDT, PIC, timer, keyboard, mouse)
* Basic process management
* Scheduler (task switching, run queue, priorities)
* System calls interface
* ELF loading support
* Virtual File System (VFS)
* Framebuffer graphics + simple compositor
* Keyboard & mouse drivers
* Basic shell interface

---

## 🧠 Architecture Overview

The project is structured as a Rust workspace:

* `kernel/` → core OS kernel (memory, scheduler, drivers, syscalls)
* `runner/` → boots and runs the OS (QEMU/bootloader integration)

---

## 🚀 How to Run

```bash
cargo run -p runner
```

> Requires Rust nightly and x86_64 target setup

---

## 🎯 Goal

To build a fully functional operating system from scratch in Rust, including:

* User space programs
* Advanced scheduling
* Filesystem support
* Windowing system
* Security model

---

## ⚠️ Status

This project is under development and not yet stable.

---

## 👨‍💻 Author

Vivek Tripathi

---

## ⭐ Note

This repository is a learning and development project focused on low-level systems and OS design.
