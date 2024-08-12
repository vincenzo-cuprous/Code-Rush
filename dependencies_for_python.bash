#! /bin/bash 
paru -S python3 --needed --moconfirm &&
python3 -m venv myenv &&
pip install pyinstaller
