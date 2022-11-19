# node-cpp
Running Native Code (C++) in JS (NodeJs) with node-gyp

## requirements
* node-gyp:
``
npm i -g node-gyp
``
* visula studio or BuildTools and cpp installed
* python 2 or later
## configuration
edit .vscode/c_cpp_properites.json to pint to your local node and buildtools installation
```
{
    "configurations": [
        {
            "name": "Win32",
            "includePath": [
                "${workspaceFolder}/**",
                "C:\\Users\\[YOUR_USER]\\AppData\\Local\\node-gyp\\Cache\\14.17.4\\include\\node"
            ],
            "defines": [
                "_DEBUG",
                "UNICODE",
                "_UNICODE"
            ],
            "windowsSdkVersion": "10.0.19041.0",
            "compilerPath": "C:/Program Files (x86)/Microsoft Visual Studio/2017/BuildTools/VC/Tools/MSVC/14.16.27023/bin/Hostx64/x64/cl.exe",
            "cStandard": "c17",
            "cppStandard": "c++17",
            "intelliSenseMode": "windows-msvc-x64"
        }
    ],
    "version": 4
}
```
## build
run configuration
```
node-gyp configure
```
build executables
```
node-gyp build
```
## run
run node app
```
node index.js
```
# expected output
for the given 500000 input
```
cpp: 14.263s
499979
JS: 23.231s
499979
```