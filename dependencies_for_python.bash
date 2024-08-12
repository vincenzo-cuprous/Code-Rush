#! /bin/bash 
paru -S python3 --needed --moconfirm &&
python3 -m venv myenv &&
paru -S fish &&
fish &&
source myenv/bin/activate.fish &&
pip install pyinstaller
