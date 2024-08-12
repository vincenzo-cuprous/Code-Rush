use std::env;
   2   │ use std::process::Command;
   3   │ use std::fs;
   4   │ 
   5   │ fn run_c(file_name: &str) {
   6   │     if !file_name.ends_with(".c") || !fs::metadata(file_name).is_ok() {
   7   │         println!("Error: File does not exist or is not a .c file.");
   8   │         return;
   9   │     }
  10   │     let executable = file_name.replace(".c", ".out");
  11   │     Command::new("gcc")
  12   │         .arg(file_name)
  13   │         .arg("-o")
  14   │         .arg(&executable)
  15   │         .status()
  16   │         .expect("Failed to compile .c file.");
  17   │     Command::new(format!("./{}", executable))
  18   │         .status()
  19   │         .expect("Failed to execute .out file.");
  20   │ }
  21   │ 
  22   │ fn run_cpp(file_name: &str) {
  23   │     if !file_name.ends_with(".cpp") || !fs::metadata(file_name).is_ok() {
  24   │         println!("Error: File does not exist or is not a .cpp file.");
  25   │         return;
  26   │     }
  27   │     let executable = file_name.replace(".cpp", ".out");
  28   │     Command::new("g++")
  29   │         .arg(file_name)
  30   │         .arg("-o")
  31   │         .arg(&executable)
  32   │         .status()
  33   │         .expect("Failed to compile .cpp file.");
  34   │     Command::new(format!("./{}", executable))
  35   │         .status()
  36   │         .expect("Failed to execute .out file.");
  37   │ }
  38   │ 
  39   │ fn run_cs(file_name: &str) {
  40   │     if !file_name.ends_with(".cs") || !fs::metadata(file_name).is_ok() {
  41   │         println!("Error: File does not exist or is not a .cs file.");
  42   │         return;
  43   │     }
  44   │     let exe_name = file_name.replace(".cs", ".exe");
  45   │     Command::new("mcs")
  46   │         .arg(file_name)
  47   │         .status()
  48   │         .expect("Failed to compile .cs file.");
  49   │     if fs::metadata(&exe_name).is_ok() {
  50   │         Command::new("mono")
  51   │             .arg(&exe_name)
  52   │             .status()
  53   │             .expect("Failed to execute .exe file.");
  54   │     } else {
  55   │         println!("Compilation failed.");
  56   │     }
  57   │ }
  58   │ 
  59   │ fn run_rb(file_name: &str) {
  60   │     if !file_name.ends_with(".rb") || !fs::metadata(file_name).is_ok() {
  61   │         println!("Error: File does not exist or is not a .rb file.");
  62   │         return;
  63   │     }
  64   │     Command::new("ruby")
  65   │         .arg(file_name)
  66   │         .status()
  67   │         .expect("Failed to run .rb file.");
  68   │ }
  69   │ 
  70   │ fn run_hx(file_name: &str) {
  71   │     if !file_name.ends_with(".hx") || !fs::metadata(file_name).is_ok() {
  72   │         println!("Error: File does not exist or is not a .hx file.");
  73   │         return;
  74   │     }
  75   │     Command::new("haxe")
  76   │         .arg("-main")
  77   │         .arg(file_name)
  78   │         .arg("--interp")
  79   │         .status()
  80   │         .expect("Failed to interpret .hx file.");
  81   │ }
  82   │ 
  83   │ fn run_dot(file_name: &str) {
  84   │     if !file_name.ends_with(".cs") || !fs::metadata(file_name).is_ok() {
  85   │         println!("Error: File does not exist or is not a .cs file.");
  86   │         return;
  87   │     }
  88   │     Command::new("dotnet")
  89   │         .arg("run")
  90   │         .arg(file_name)
  91   │         .status()
  92   │         .expect("Failed to run .cs file.");
  93   │ }
  94   │ 
  95   │ fn run_golang(file_name: &str) {
  96   │     if !file_name.ends_with(".go") || !fs::metadata(file_name).is_ok() {
  97   │         println!("Error: File does not exist or is not a .go file.");
  98   │         return;
  99   │     }
 100   │     Command::new("go")
 101   │         .arg("run")
 102   │         .arg(file_name)
 103   │         .status()
 104   │         .expect("Failed to run .go file.");
 105   │ }
 106   │ 
 107   │ fn run_jul(file_name: &str) {
 108   │     if !file_name.ends_with(".jl") || !fs::metadata(file_name).is_ok() {
 109   │         println!("Error: File does not exist or is not a .jl file.");
 110   │         return;
 111   │     }
 112   │     Command::new("julia")
 113   │         .arg(file_name)
 114   │         .status()
 115   │         .expect("Failed to run .jl file.");
 116   │ }
 117   │ 
 118   │ fn run_java(file_name: &str) {
 119   │     if !file_name.ends_with(".java") || !fs::metadata(file_name).is_ok() {
 120   │         println!("Error: File does not exist or is not a .java file.");
 121   │         return;
 122   │     }
 123   │     let base_name = file_name.replace(".java", "");
 124   │     Command::new("javac")
 125   │         .arg(file_name)
 126   │         .status()
 127   │         .expect("Failed to compile .java file.");
 128   │     Command::new("java")
 129   │         .arg(base_name)
 130   │         .status()
 131   │         .expect("Failed to run .class file.");
 132   │ }
 133   │ 
 134   │ fn run_py(file_name: &str) {
 135   │     if !file_name.ends_with(".py") || !fs::metadata(file_name).is_ok() {
 136   │         println!("Error: File does not exist or is not a .py file.");
 137   │         return;
 138   │     }
 139   │     Command::new("python3")
 140   │         .arg(file_name)
 141   │         .status()
 142   │         .expect("Failed to run .py file.");
 143   │ }
 144   │ 
 145   │ fn run_z(file_name: &str) {
 146   │     if !file_name.ends_with(".zig") || !fs::metadata(file_name).is_ok() {
 147   │         println!("Error: File does not exist or is not a .zig file.");
 148   │         return;
 149   │     }
 150   │     Command::new("zig")
 151   │         .arg("run")
 152   │         .arg(file_name)
 153   │         .status()
 154   │         .expect("Failed to run .zig file.");
 155   │ }
 156   │ 
 157   │ fn run_n(file_name: &str) {
 158   │     if !file_name.ends_with(".nim") || !fs::metadata(file_name).is_ok() {
 159   │         println!("Error: File does not exist or is not a .nim file.");
 160   │         return;
 161   │     }
 162   │     Command::new("nim")
 163   │         .arg("c")
 164   │         .arg("-r")
 165   │         .arg("--verbosity:0")
 166   │         .arg(file_name)
 167   │         .status()
 168   │         .expect("Failed to compile and run .nim file.");
 169   │ }
 170   │ 
 171   │ fn run_rust(file_name: &str) {
 172   │     if !file_name.ends_with(".rs") || !fs::metadata(file_name).is_ok() {
 173   │         println!("Error: File does not exist or is not a .rs file.");
 174   │         return;
 175   │     }
 176   │     let executable = file_name.replace(".rs", "");
 177   │     Command::new("rustc")
 178   │         .arg(file_name)
 179   │         .status()
 180   │         .expect("Failed to compile .rs file.");
 181   │     Command::new(format!("./{}", executable))
 182   │         .status()
 183   │         .expect("Failed to execute .rs file.");
 184   │ }
 185   │ 
 186   │ fn run_kotlin(file_name: &str) {
 187   │     if !file_name.ends_with(".kt") || !fs::metadata(file_name).is_ok() {
 188   │         println!("Error: File does not exist or is not a .kt file.");
 189   │         return;
 190   │     }
 191   │     let jar_file = file_name.replace(".kt", ".jar");
 192   │     let compile_result = Command::new("kotlinc")
 193   │         .arg(file_name)
 194   │         .arg("-include-runtime")
 195   │         .arg("-d")
 196   │         .arg(&jar_file)
 197   │         .output()
 198   │         .expect("Failed to compile .kt file.");
 199   │     if !compile_result.status.success() {
 200   │         println!(
 201   │             "Compilation failed:\n{}",
 202   │             String::from_utf8_lossy(&compile_result.stderr)
 203   │         );
 204   │         return;
 205   │     }
 206   │     let run_result = Command::new("kotlin")
 207   │         .arg(jar_file)
 208   │         .output()
 209   │         .expect("Failed to run .jar file.");
 210   │     if !run_result.status.success() {
 211   │         println!(
 212   │             "Execution failed:\n{}",
 213   │             String::from_utf8_lossy(&run_result.stderr)
 214   │         );
 215   │     } else {
 216   │         println!("Program output:\n{}", String::from_utf8_lossy(&run_result.stdou
       │ t));
 217   │     }
 218   │ }
 219   │ 
 220   │ fn main() {
 221   │     let args: Vec<String> = env::args().collect();
 222   │     if args.len() != 2 {
 223   │         eprintln!("Usage: run_file <filename>");
 224   │         std::process::exit(1);
 225   │     }
 226   │ 
 227   │     let file_name = &args[1];
 228   │ 
 229   │     match file_name.as_str() {
 230   │         f if f.ends_with(".c") => run_c(f),
 231   │         f if f.ends_with(".cpp") => run_cpp(f),
 232   │         f if f.ends_with(".cs") => run_cs(f),
 233   │         f if f.ends_with(".rb") => run_rb(f),
 234   │         f if f.ends_with(".hx") => run_hx(f),
 235   │         f if f.ends_with(".go") => run_golang(f),
 236   │         f if f.ends_with(".jl") => run_jul(f),
 237   │         f if f.ends_with(".java") => run_java(f),
 238   │         f if f.ends_with(".py") => run_py(f),
 239   │         f if f.ends_with(".zig") => run_z(f),
 240   │         f if f.ends_with(".nim") => run_n(f),
 241   │         f if f.ends_with(".rs") => run_rust(f),
 242   │         f if f.ends_with(".kt") => run_kotlin(f),
 243   │         _ => eprintln!("Error: Unsupported file type."),
 244   │     }
 245   │ }
