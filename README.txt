                ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                 DISKDEVIL - DEVILISHLY FAST ISO TO USB

                            Soham S Gumaste
                ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━


Table of Contents
─────────────────

1. Introduction
.. 1. Installing
2. FAQ
.. 1. Doesn't DD already do this?
.. 2. Can't you write a syscall wrapper in like 5 lines?
.. 3. What's all this ./configure crap?
.. 4. Why write it in Rust?
.. 5. How can you get the safety of rust if theres a bunch of inline asm and everything is unsafe?
.. 6. Why is the readme looking weird?
3. LICENSE


――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――


1 Introduction
══════════════

  Diskdevil is a fancy wrapper for the `sendfile64' system call on
  linux, enabling super fast zero-copy file transfers. Learn more about
  sendfile by consulting your local `man` command.


1.1 Installing
──────────────

  Grab a release tarball, and
  ┌────
  │ ./configure && make && make install
  └────
  You will need rust target `x86_64-unknown-none'.


2 FAQ
═════

2.1 Doesn't DD already do this?
───────────────────────────────

  Well yes, and that is the inspiration for the name
  _d_​isk​_d_​evil. However, `dd' is approximately 25 centuries old and
  only uses read/write system calls [1].  Diskdevil does not (and
  probably will not) support all of the disk-destroying capabilities of
  `dd', but *should* be faster at what basically everyone uses `dd' for,
  to dump Linux ISOs to USBs.


2.2 Can't you write a syscall wrapper in like 5 lines?
──────────────────────────────────────────────────────

  Also yes, but I wanted a challege (read: procrastinate) so I decided
  to make it `no_std' ie no rust standard lib apart from `core', no
  `libc', no nothing. Just me and some inline assembly.

  I also wrote it _WITHOUT_ cargo for……reasons.


2.3 What's all this ./configure crap?
─────────────────────────────────────

  What's [Cargo]?


[Cargo] <./Cargo.toml>


2.4 Why write it in Rust?
─────────────────────────

  The white house told me to? Safety or something.


2.5 How can you get the safety of rust if theres a bunch of inline asm and everything is unsafe?
────────────────────────────────────────────────────────────────────────────────────────────────

  What's with all these questions?


2.6 Why is the readme looking weird?
────────────────────────────────────

  Github's support for Org Mode is broken, so plain text it is..

  ――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――


3 LICENSE
═════════

  [GPLv3] especially the "AS-IS" and "no warranty" parts.


[GPLv3] <./LICENSE>



Footnotes
─────────

[1] : Citation needed.
