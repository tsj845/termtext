from os import walk, system

for fname in walk(".").__next__()[2]:
    if fname.endswith(".asm"):
        ecode = system(f"nasm -fmacho64 {fname}")
        print(f"Assembling \"{fname}\", exit code: {ecode}")