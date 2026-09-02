## core

The heart of Kalopsia, this is where the TUI lives in, and where every single call is done.

Also, the core is the only crate that actually has dependencies, which are `ratatui` and `crossterm`.

> Sadly, I need those dependencies, I'm not writing a complex frontend terminal renderer, at least not yet.

---

Small script for VMBL, defining myself (here just to save it, but will be removed):

```vmbl
DEFINE NODE (
    name="voidstar"; 
    difficulty=9; 
    hours=100; 
    req_skills=["Assembly", "Rust", "LLVM", "Self-Hosting"]; 
    gain_skills=["LLVM", "Rust", "Compilers", "Assembly", "NASM"]
);
DEFINE NODE (
    name="Lightweight Dock"; 
    difficulty=7; 
    hours=80; 
    req_skills=["Rust", "Syscalls", "Explorer", "Operating Systems"]; 
    gain_skills=["Operating Systems", "Syscalls", "Core System Functions"]
);

DEFINE OBJ (
    skills=[
        "Rust", 
        "Bytecode", 
        "Assembly", 
        "CSharp", 
        "Dotnet", 
        "Java", 
        "Spring", 
        "Interpreters", 
        "VM Stack-Based Compilers" -- will add more stuff soon enough
        ]
    );
```