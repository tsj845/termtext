from os import walk, system

e = False

for fname in walk(".").__next__()[2]:
    if fname.endswith(".asm"):
        ecode = system(f"nasm -fmacho64 {fname}")
        print(f"Assembling \"{fname}\", exit code: {ecode}")
        e = e or ecode
    # elif fname.endswith(".h"):
        # ecode = system(f"clang -c {fname} --target=x86_64 -o {fname[:-2]+'.pout'} -I/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/Kernel.framework/Versions/Current/Headers/")
        # print(f"Compiling C Header File \"{fname}\", exit code: {ecode}")
        # fname = fname[:-2]+".asm"
        # ecode = system(f"nasm -fmacho64 {fname}")
        # print(f"Assembling \"{fname}\", exit code: {ecode}")

if (e):
    exit(1)