import os
import sys

os.chdir(os.path.join(__file__, os.pardir))
x = os.system("cargo build --release")
if x != 0: sys.exit(x)
