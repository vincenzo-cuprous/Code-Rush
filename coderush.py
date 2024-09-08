import sys
import subprocess
import os

# Define the config file path in the user's home directory
CONFIG_DIR = os.path.expanduser('~/.config/coderush')
CONFIG_FILE = os.path.join(CONFIG_DIR, 'compiler_settings.txt')

def ensure_config_dir():
    """Ensure the config directory exists."""
    if not os.path.exists(CONFIG_DIR):
        os.makedirs(CONFIG_DIR)

def set_default_cs_compiler(compiler):
    """Set the default C# compiler and store it in the config file."""
    if compiler not in ['dotnet', 'mono']:
        print("Error: Unsupported compiler. Use 'dotnet' or 'mono'.")
        return
    
    ensure_config_dir()  # Ensure the config directory exists
    with open(CONFIG_FILE, 'w') as f:
        f.write(compiler)
    print(f"Default C# compiler set to: {compiler}")

def get_default_cs_compiler():
    """Retrieve the default C# compiler from the config file."""
    if not os.path.isfile(CONFIG_FILE):
        return 'mono'  # Default to mono if no config file exists
    
    with open(CONFIG_FILE, 'r') as f:
        compiler = f.read().strip()
    
    if compiler not in ['dotnet', 'mono']:
        return 'mono'  # Fallback to mono if invalid data is in the config file
    return compiler

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
    """Run the C# file with the default or selected compiler."""
    if not os.path.isfile(file_name) or not file_name.endswith('.cs'):
        print("Error: File does not exist or is not a .cs file.")
        return

    exe_name = file_name.replace('.cs', '.exe')
    cs_compiler = get_default_cs_compiler()

    if cs_compiler == 'mono':
        # Use mcs and run with mono
        subprocess.run(['mcs', file_name])
        if os.path.isfile(exe_name):
            subprocess.run(['mono', exe_name])
        else:
            print("Compilation failed")
    elif cs_compiler == 'dotnet':
        # Use dotnet to run the .cs file
        subprocess.run(['dotnet', 'run', file_name])

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

def run_kotlin(file_name):
    if not os.path.isfile(file_name) or not file_name.endswith('.kt'):
        print("Error: File does not exist or is not a .kt file.")
        return
    jar_file = file_name.replace('.kt', '.jar')
    compile_command = ['kotlinc', file_name, '-include-runtime', '-d', jar_file]
    compile_result = subprocess.run(compile_command, capture_output=True, text=True)
    if compile_result.returncode != 0:
        print("Compilation failed:\n", compile_result.stderr)
        return
    run_command = ['kotlin', jar_file]
    run_result = subprocess.run(run_command, capture_output=True, text=True)
    if run_result.returncode != 0:
        print("Execution failed:\n", run_result.stderr)
    else:
        print("Program output:\n", run_result.stdout)

def main():
    if len(sys.argv) < 2:
        print("Usage: coderush [-c compiler] <filename>")
        sys.exit(1)

    # Check for compiler option
    if sys.argv[1] == '-c' and len(sys.argv) > 2:
        compiler = sys.argv[2]
        set_default_cs_compiler(compiler)
        sys.exit(0)  # Exit after setting the compiler

    file_name = sys.argv[1]

    # Decide which run function to call based on file extension
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
    elif file_name.endswith('.kt'):
        run_kotlin(file_name)
    else:
        print("Error: Unsupported file type.")

if __name__ == "__main__":
    main()
