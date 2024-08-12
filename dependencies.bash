#! /bin/bash
paru -S aur/pyinstaller --needed --noconfirm && 
paru -S python3 --needed --moconfirm &&
paru -S rustup && 
rustup default stable &&
python3 -m venv myenv &&
pip install pyinstaller
