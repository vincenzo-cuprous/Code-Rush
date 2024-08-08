import sys
import subprocess
import os

def run_c(file_name):
    if not os.path.isfile(file_name) or not file_name.endswith('.c'):
        print("Error: File does not exist or is not a .c file.")
        return
    executable = file_name.replace('.c', '.out')
    subprocess.run(['gcc', file_name, '-o', executable])
    subprocess.run([f'./{executable}'])

def run_cpp(file_name):
    if not os.path.isfile(file_name) or not file_name.endswith('.cpp'):
        print("Error: File does not exist or is not a .cpp file.")
        return
    executable = file_name.replace('.cpp', '.out')
    subprocess.run(['g++', file_name, '-o', executable])
    subprocess.run([f'./{executable}'])

def run_cs(file_name):
    if not os.path.isfile(file_name) or not file_name.endswith('.cs'):
        print("Error: File does not exist or is not a .cs file.")
        return
    exe_name = file_name.replace('.cs', '.exe')
    subprocess.run(['mcs', file_name])
    if os.path.isfile(exe_name):
        subprocess.run(['mono', exe_name])
    else:
        print("Compilation failed")

def run_rb(file_name):
    if not os.path.isfile(file_name) or not file_name.endswith('.rb'):
        print("Error: File does not exist or is not a .rb file.")
        return
    subprocess.run(['ruby', file_name])

def run_hx(file_name):
    if not os.path.isfile(file_name) or not file_name.endswith('.hx'):
        print("Error: File does not exist or is not a .hx file.")
        return
    subprocess.run(['haxe', '-main', file_name, '--interp'])

def run_dot(file_name):
    if not os.path.isfile(file_name) or not file_name.endswith('.cs'):
        print("Error: File does not exist or is not a .cs file.")
        return
    subprocess.run(['dotnet', 'run', file_name])

def run_golang(file_name):
    if not os.path.isfile(file_name) or not file_name.endswith('.go'):
        print("Error: File does not exist or is not a .go file.")
        return
    subprocess.run(['go', 'run', file_name])

def run_jul(file_name):
    if not os.path.isfile(file_name) or not file_name.endswith('.jl'):
        print("Error: File does not exist or is not a .jl file.")
        return
    subprocess.run(['julia', file_name])

def run_java(file_name):
    if not os.path.isfile(file_name) or not file_name.endswith('.java'):
        print("Error: File does not exist or is not a .java file.")
        return
    base_name = os.path.basename(file_name).replace('.java', '')
    subprocess.run(['javac', file_name])
    subprocess.run(['java', base_name])

def run_py(file_name):
    if not os.path.isfile(file_name) or not file_name.endswith('.py'):
        print("Error: File does not exist or is not a .py file.")
        return
    subprocess.run(['python3', file_name])

def run_z(file_name):
    if not os.path.isfile(file_name) or not file_name.endswith('.zig'):
        print("Error: File does not exist or is not a .zig file.")
        return
    subprocess.run(['zig', 'run', file_name])

def run_n(file_name):
    if not os.path.isfile(file_name) or not file_name.endswith('.nim'):
        print("Error: File does not exist or is not a .nim file.")
        return
    subprocess.run(['nim', 'c', '-r', '--verbosity:0', file_name])

def run_rust(file_name):
    if not os.path.isfile(file_name) or not file_name.endswith('.rs'):
        print("Error: File does not exist or is not a .rs file.")
        return
    executable = file_name.replace('.rs', '')
    subprocess.run(['rustc', file_name])
    subprocess.run([f'./{executable}'])

def main():
    if len(sys.argv) != 2:
        print("Usage: run_file.py <filename>")
        sys.exit(1)
    
    file_name = sys.argv[1]
    
    if file_name.endswith('.c'):
        run_c(file_name)
    elif file_name.endswith('.cpp'):
        run_cpp(file_name)
    elif file_name.endswith('.cs'):
        run_cs(file_name)
    elif file_name.endswith('.rb'):
        run_rb(file_name)
    elif file_name.endswith('.hx'):
        run_hx(file_name)
    elif file_name.endswith('.cs'):
        run_dot(file_name)
    elif file_name.endswith('.go'):
        run_golang(file_name)
    elif file_name.endswith('.jl'):
        run_jul(file_name)
    elif file_name.endswith('.java'):
        run_java(file_name)
    elif file_name.endswith('.py'):
        run_py(file_name)
    elif file_name.endswith('.zig'):
        run_z(file_name)
    elif file_name.endswith('.nim'):
        run_n(file_name)
    elif file_name.endswith('.rs'):
        run_rust(file_name)
    else:
        print("Error: Unsupported file type.")

if __name__ == "__main__":
    main()
