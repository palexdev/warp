# warp

![GitHub Release](https://img.shields.io/github/v/release/palexdev/warp)
![GitHub Downloads](https://img.shields.io/github/downloads/palexdev/warp/latest/total)
![GitHub Downloads](https://img.shields.io/github/downloads/palexdev/warp/total)
![License](https://img.shields.io/github/license/palexdev/warp)

A fork of [Warp](https://github.com/kirbylink/warp) by [@kirbylink](https://github.com/kirbylink) with a bunch of fixes, major performance improvements, and some new features.

Original work by [dgiagio](https://github.com/dgiagio/warp).

---

## Table of Contents

- [Changes in v1.4.0](#changes-in-v140)
- [Changes in v1.3.0](#changes-in-v130)
- [Changes in v1.2.0](#changes-in-v120)
- [Changes in v1.0.0](#changes-in-v100)
- [Prebuilt Binaries](#prebuilt-binaries)
- [Supported Platforms](#supported-platforms)
- [Quickstart with Node.js](#quickstart-with-nodejs)
  - [Linux](#linux)
  - [macOS](#macos)
  - [Windows](#windows)
- [Quickstart with .NET Core](#quickstart-with-net-core)
  - [Linux](#linux-1)
  - [macOS](#macos-1)
  - [Windows](#windows-1)
- [Quickstart with Java](#quickstart-with-java)
- [How it Works](#how-it-works)
- [macOS Security Considerations](#macos-security-considerations)
- [Authors](#authors)
- [License](#license)

---

## Changes in v1.4.0

### New CLI Options
* `--no-persist`: Extract the application into a new temporary directory on every run and delete it when the application exits. Nothing is kept in the warp cache. Cannot be combined with `--unique-id`.
* `--arch auto`: `--arch` is now optional and defaults to `auto`, which packs for the platform the packer runs on. It selects which runner is embedded, so it describes where the binary will run, not where it is packed. Passing a platform explicitly still allows packing for other platforms, for example `--arch windows-x64` on Linux, as long as the input directory contains the application built for that platform. `warp-packer list` marks the platform `auto` resolves to.
* `--warp-clean` (on the packed binary, not the packer): Running `./app --warp-clean` deletes the application's cache directory instead of starting it. See [Cleaning the Cache](#cleaning-the-cache).

### Improvements
* **Faster first run:** The payload is split into 2 MiB chunks compressed with zstd (level 19) instead of a single gzip-compressed tar archive. The runner decompresses all chunks in parallel and writes each one directly to its final position.
* **Smaller binaries:** zstd at level 19 produces smaller payloads than gzip at its best level.
* **Faster warm starts:** A fixed footer at the end of the binary points to the arguments and the file index, so the runner no longer scans its own binary on every launch.
* **Reliable updates:** The cache is validated with a hash of the payload instead of modification times. A new version always replaces the old one, even when its file has an older timestamp, and two applications with the same file name no longer run each other's files.
* **Safe extraction:** Files are extracted into a temporary directory that is renamed into place under a file lock. An interrupted extraction is redone on the next launch, and concurrent first launches no longer interfere with each other.
* **Ctrl+C handling:** The runner waits for the application to exit instead of terminating immediately, and on Unix reports a termination by signal as exit code 128 + signal.
* **Constant memory use:** The runner reads each chunk into a small buffer instead of memory-mapping the whole binary. Peak memory during extraction stays around 50 MB, no matter how large the application is.
* **Easier `--exec`:** The executable can be given relative to the input directory as before, or as a path from the current directory or an absolute path inside the input directory. For example, `-i myapp -e myapp/bin/run.sh` now works.

### Fixes
* Packing from an NTFS drive on Linux no longer stores small files as zeros.
* Packing over an existing, larger output file no longer leaves stale bytes at the end of the binary.
* `--exec` paths pointing outside the input directory are rejected instead of producing a binary that cannot find its executable.

### Performance
Measured on Linux x64 with a 45 MB jlink image:

| | v1.3.0 | v1.4.0 |
|---|---|---|
| Binary size | 23.5 MB | 21.7 MB |
| First run, extraction included | 294 ms | 188 ms |
| Extraction alone | about 114 ms | about 13 ms |
| First run with a cold page cache | 303 ms | 200 ms |
| Warm run | 180 ms | 176 ms |
| Run with `--no-persist` | n/a | 197 ms |

Larger applications, first run with an empty cache:

| | v1.3.0 | v1.4.0 |
|---|---|---|
| 130 MB Java desktop app | 407 ms | 84 ms |
| 583 MB Electron app | 1641 ms | 190 ms |
| Peak memory during extraction, 583 MB Electron app | 43 MB | 52 MB |

The extraction speedup depends on the number of CPU cores.

v1.3.0 used slightly less memory because it decompressed the payload as a single stream on one thread. v1.4.0 decompresses all chunks in parallel and needs a buffer per thread, which costs a few MB but keeps peak memory constant regardless of the application size.

### Compatibility
Binaries created with v1.4.0 use a new payload format. Binaries created with older versions keep working, because every binary contains its own runner.

---

## Changes in v1.3.0

### New CLI Option
* `--hidden`: Prevent the command prompt (console) from flashing when starting an application (Windows).

### Improvements
* **Enhanced Silent Mode:** Combined with batch scripts, the runner now ensures that no terminal window remains visible in the background.
* **Backward Compatible Metadata:** Updated the internal communication between packer and runner to be more robust for future updates.

---

## Changes in v1.2.0

### Improvements

* Added **macOS Universal binary support** (`universal2-apple-darwin`), allowing the same executable to run on both Intel and Apple Silicon Macs.
* Simplified the cross-compilation toolchain by switching fully to **cargo-zigbuild** and **Zig**, removing the need for `osxcross`, macOS SDK downloads, and additional signing tools.

---

## Changes in v1.0.0

This fork introduces several improvements and compatibility fixes compared to the original [Warp repository](https://github.com/dgiagio/warp). For a full comparison, see the [diff](https://github.com/dgiagio/warp/compare/master...kirbylink:warp:master).

### Breaking Changes

The default invocation has changed from:

```bash
warp-packer <args>
````

to:

```bash
warp-packer pack <args>
```

### New CLI Options

* `-i, --input-dir`: Directory with your application and dependencies
* `-q, --unique-id`: Creates a unique ID for each package (for testing/multiple versions)
* `-p, --prefix`: Set a custom name for the extraction directory
* `-n, --no-clean`: Prevents overwriting extracted versions in the cache

---

## Prebuilt Binaries

Prebuilt `warp-packer` binaries are available on the
[GitHub Releases](https://github.com/palexdev/warp/releases) page.

Available platforms:

- Linux (x64, aarch64)
- Windows (x64, aarch64)
- macOS (x64, aarch64)

Example download (Linux x64):

```bash
curl -Lo warp-packer \
https://github.com/palexdev/warp/releases/latest/download/linux-x64.warp-packer

chmod +x warp-packer
```

---

## Supported Platforms

| Platform | Architecture | Binary |
|--------|--------|--------|
| Linux | x64 | ✅ |
| Linux | ARM64 | ✅ |
| Windows | x64 | ✅ |
| Windows | ARM64 | ✅ |
| macOS | Intel | ✅ |
| macOS | Apple Silicon | ✅ |

---

## Quickstart with Node.js
### Linux
**Create the directory for the application**
```sh
mkdir myapp
cd myapp
```

**Create main application** - `app.js`
```javascript
var lodash = require('lodash');
var output = lodash.without([1, 2, 3], 1);
console.log(output);
```

**Download Node.js distribution**
```sh
wget https://nodejs.org/dist/v8.12.0/node-v8.12.0-linux-x64.tar.xz
xz -dc node-v8.12.0-linux-x64.tar.xz | tar xvf -
```

**Install dependencies**
```sh
node-v8.12.0-linux-x64/bin/npm install lodash
```

**Remove unneeded files**
```sh
rm -r node-v8.12.0-linux-x64/include node-v8.12.0-linux-x64/share node-v8.12.0-linux-x64/lib
rm node-v8.12.0-linux-x64/bin/npm node-v8.12.0-linux-x64/bin/npx
```

**Create launcher script** - `launch`
```sh
#!/bin/sh

NODE_DIST=node-v8.12.0-linux-x64
APP_MAIN_JS=app.js

DIR="$(cd "$(dirname "$0")" ; pwd -P)"
NODE_EXE=$DIR/$NODE_DIST/bin/node
NODE_PATH=$DIR/node_modules
APP_MAIN_JS_PATH=$DIR/$APP_MAIN_JS

exec $NODE_EXE $APP_MAIN_JS_PATH $@
```

**Make the launcher script executable**
```sh
chmod +x launch
```

**Download `warp-packer`**

If you save `warp-packer` in a directory in your PATH, you only need to download it once.
```sh
cd ..
curl -Lo warp-packer https://github.com/palexdev/warp/releases/latest/download/linux-x64.warp-packer
chmod +x warp-packer
```

**Create your self-contained application**

```sh
./warp-packer pack --arch linux-x64 --input-dir myapp --exec launch --output myapp.bin
chmod +x myapp.bin
```

**Run your self-contained application**

```sh
./myapp.bin
[ 2, 3 ]
```

**More information about your self-contained application**

```sh
file myapp.bin
myapp.bin: ELF 64-bit LSB shared object, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, for GNU/Linux 3.2.0, BuildID[sha1]=aa53b01be2cde5e0b64450870b1af13b52d5cffb, with debug_info, not stripped

du -hs myapp.bin
17M     myapp.bin
```

### macOS
**Create the directory for the application**
```sh
mkdir myapp
cd myapp
```

**Create main application** - `app.js`
```javascript
var lodash = require('lodash');
var output = lodash.without([1, 2, 3], 1);
console.log(output);
```

**Download Node.js distribution**
```sh
curl -Lo node-v8.12.0-darwin-x64.tar.gz https://nodejs.org/dist/v8.12.0/node-v8.12.0-darwin-x64.tar.gz
tar xvfz node-v8.12.0-darwin-x64.tar.gz
```

**Install dependencies**
```sh
PATH=node-v8.12.0-darwin-x64/bin npm install lodash
```

**Remove unneeded files**
```sh
rm -r node-v8.12.0-darwin-x64/include node-v8.12.0-darwin-x64/share node-v8.12.0-darwin-x64/lib
rm node-v8.12.0-darwin-x64/bin/npm node-v8.12.0-darwin-x64/bin/npx
```

**Create launcher script*** - `launch`
```sh
#!/bin/sh

NODE_DIST=node-v8.12.0-darwin-x64
APP_MAIN_JS=app.js

DIR="$(cd "$(dirname "$0")" ; pwd -P)"
NODE_EXE=$DIR/$NODE_DIST/bin/node
NODE_PATH=$DIR/node_modules
APP_MAIN_JS_PATH=$DIR/$APP_MAIN_JS

exec "$NODE_EXE" "$APP_MAIN_JS_PATH" $@
```

**Make the launcher script executable**
```sh
chmod +x launch
```

**Download `warp-packer`**

If you save `warp-packer` in a directory in your PATH, you only need to download it once.
```sh
cd ..
curl -Lo warp-packer https://github.com/palexdev/warp/releases/latest/download/macos-x64.warp-packer
chmod +x warp-packer
```

**Create your self-contained application**

```sh
./warp-packer pack --arch macos-x64 --input-dir myapp --exec launch --output myapp.bin
chmod +x myapp.bin
```

**Run your self-contained application**

```sh
./myapp.bin
[ 2, 3 ]
```

**More information about your self-contained application**

```sh
file myapp.bin
myapp.bin: Mach-O 64-bit executable x86_64

du -hs myapp.bin
26M     myapp.bin
```

### Windows
**Create the directory for the application**
```powershell
mkdir myapp
cd myapp
```

**Create main application** - `app.js`
```javascript
var lodash = require('lodash');
var output = lodash.without([1, 2, 3], 1);
console.log(output);
```

**Download Node.js distribution**
```powershell
curl https://nodejs.org/dist/v8.12.0/node-v8.12.0-win-x64.zip -OutFile node-v8.12.0-win-x64.zip
Expand-Archive .\node-v8.12.0-win-x64.zip -DestinationPath .\
```

**Install dependencies**
```powershell
.\node-v8.12.0-win-x64\npm install lodash
```

**Remove unneeded files**
```powershell
rmdir -Recurse .\node-v8.12.0-win-x64\node_modules\npm
```

**Create launcher script*** - `launch.cmd`
```bat
@ECHO OFF

SETLOCAL

SET "NODE_DIST=node-v8.12.0-win-x64"
SET "APP_MAIN_JS=app.js"

SET "NODE_EXE=%~dp0\%NODE_DIST%\node.exe"
SET "NODE_PATH=%~dp0\%NODE_DIST%\node_modules"
SET "APP_MAIN_JS_PATH=%~dp0\%APP_MAIN_JS%"

CALL %NODE_EXE% %APP_MAIN_JS_PATH% %*
EXIT /B %ERRORLEVEL%
```

**Download `warp-packer`**

If you save `warp-packer` in a directory in your PATH, you only need to download it once.
```powershell
cd ..
[Net.ServicePointManager]::SecurityProtocol = "tls12, tls11, tls" ; Invoke-WebRequest https://github.com/palexdev/warp/releases/latest/download/windows-x64.warp-packer.exe -OutFile warp-packer.exe
```

**Create your self-contained application**

```powershell
.\warp-packer --arch windows-x64 --input-dir .\myapp\ --exec launch.cmd --output myapp.exe
```

**Run your self-contained application**

```powershell
.\myapp.exe
[ 2, 3 ]
PS $HOME>
```

**More information about your self-contained application**

```powershell
"{0:N2} MB" -f ((Get-Item myapp.exe).Length / 1MB)
9.15 MB
```

---

## Quickstart with .NET Core
### Linux
**Create a simple console application**

```sh
mkdir myapp
cd myapp
dotnet new console
dotnet run
Hello World!
```

**Publish the application with native installer for `linux-x64` runtime**

```sh
dotnet publish -c Release -r linux-x64
```
The application should be published to `bin/Release/netcoreapp2.1/linux-x64/publish/`

**Download `warp-packer`**

If you save `warp-packer` in a directory in your PATH, you only need to download it once.
```sh
curl -Lo warp-packer https://github.com/palexdev/warp/releases/latest/download/linux-x64.warp-packer
chmod +x warp-packer
```

**Create your self-contained application**

```sh
./warp-packer pack --arch linux-x64 --input-dir bin/Release/netcoreapp2.1/linux-x64/publish --exec myapp --output myapp
chmod +x myapp
```

**Run your self-contained application**

```sh
./myapp
Hello World!
```

**More information about your self-contained application**

```sh
file myapp
myapp: ELF 64-bit LSB executable, x86-64, version 1 (GNU/Linux), statically linked, BuildID[sha1]=13b12e71a63ca1de8537ad7e90c83241f9f87f6c, with debug_info, not stripped

du -hs myapp
34M     myapp
```

### macOS
**Create a simple console application**

```sh
mkdir myapp
cd myapp
dotnet new console
dotnet run
Hello World!
```

**Publish the application with native installer for `osx-x64` runtime**

```sh
dotnet publish -c Release -r osx-x64
```
The application should be published to `bin/Release/netcoreapp2.1/osx-x64/publish/`

**Download `warp-packer`**

If you save `warp-packer` in a directory in your PATH, you only need to download it once.
```sh
curl -Lo warp-packer https://github.com/palexdev/warp/releases/latest/download/macos-x64.warp-packer
chmod +x warp-packer
```

**Create your self-contained application**

```sh
./warp-packer pack --arch macos-x64 --input-dir bin/Release/netcoreapp2.1/osx-x64/publish --exec myapp --output myapp
chmod +x myapp
```

**Run your self-contained application**

```sh
./myapp
Hello World!
```

**More information about your self-contained application**

```sh
file myapp
myapp: Mach-O 64-bit executable x86_64

du -hs myapp
 27M    myapp
```

### Windows
**Create a simple console application**

```powershell
mkdir myapp
cd myapp
dotnet new console
dotnet run
Hello World!
PS $HOME\myapp>
```

**Publish the application with native installer for `win10-x64` runtime**

```powershell
dotnet publish -c Release -r win10-x64
```
The application should be published to `bin/Release/netcoreapp2.1/win10-x64/publish/`

**Download `warp-packer`**

If you save `warp-packer` in a directory in your PATH, you only need to download it once.
```powershell
[Net.ServicePointManager]::SecurityProtocol = "tls12, tls11, tls" ; Invoke-WebRequest https://github.com/palexdev/warp/releases/latest/download/windows-x64.warp-packer.exe -OutFile warp-packer.exe
```

**Create your self-contained application**

```powershell
.\warp-packer --arch windows-x64 --input-dir bin/Release/netcoreapp2.1/win10-x64/publish --exec myapp.exe --output myapp.exe
```

**Run your self-contained application**

```powershell
.\myapp.exe
Hello World!
PS $HOME\myapp>
```

**More information about your self-contained application**

```powershell
"{0:N2} MB" -f ((Get-Item myapp.exe).Length / 1MB)
28.51 MB
```

---

## Quickstart with Java

Warp can package Java JARs along with a minimal JRE and startup scripts.

> 🧠 **Tip:** If you want to skip the manual steps below, check out
> [`java-warp4j`](https://github.com/kirbylink/java-warp4j) — a Java-based tool that automates all of this:
> It handles JDK downloads, `jlink`, script creation, and even runs `warp-packer` internally.

### Manual Instructions

1. **Create a Hello World app**:

```java
// HelloWorld.java
public final class HelloWorld {
  public static void main(final String[] args) {
    System.out.println("Hello, world.");
  }
}
```

2. **Compile and bundle as JAR**:

```bash
javac HelloWorld.java
jar cvfe app.jar HelloWorld HelloWorld.class
```

3. **Download and unpack a JRE** (e.g., from [Adoptium](https://adoptium.net)):

```bash
wget https://.../OpenJDK8U-jre_x64_linux_hotspot_8u412b08.tar.gz
tar -xvf OpenJDK8U-jre_x64_linux_hotspot_8u412b08.tar.gz
```

4. **Create a bundle folder**:

```bash
mkdir bundle
cp -r jdk-folder bundle/jre
cp app.jar bundle/
```

5. **Add a launcher script** (`bundle/run.sh`):

```bash
#!/bin/bash
DIR="$(cd "$(dirname "$0")"; pwd)"
exec "$DIR/jre/bin/java" -jar "$DIR/app.jar" "$@"
```

6. **Make script executable**:

```bash
chmod +x bundle/run.sh
```

7. **Download warp-packer** (if not already in PATH):

```bash
curl -Lo warp-packer https://github.com/palexdev/warp/releases/latest/download/linux-x64.warp-packer
chmod +x warp-packer
```

8. **Create your binary**:

```bash
./warp-packer pack --arch linux-x64 --input-dir bundle --exec run.sh --output app.bin
chmod +x app.bin
```

9. **Run your binary**:

```bash
./app.bin
```

---

## How it Works

Warp combines your runtime and application into a single file using two tools:

* **`warp-runner`**: The runtime stub that extracts and launches your app
* **`warp-packer`**: CLI tool that builds the binary, compresses files, and fetches `warp-runner`

<img src="documentation/warp-app-binary.png" width="272">

### Binary Layout

A packed binary consists of the runner, followed by the compressed file chunks, the packer arguments, the file index, and a fixed-size footer. The footer stores where the arguments and the index are located, so the runner can read them directly without scanning the file.

### First-time Run

The first execution unpacks your app to a cache directory and runs it. Subsequent executions run directly. The cache is replaced automatically when the binary contains a different payload.

With `--no-persist`, the app is unpacked into a new temporary directory on every run, and the directory is deleted when the app exits.

### Cache Directories

* Linux: `$HOME/.local/share/warp/`
* macOS: `$HOME/Library/Application Support/warp/`
* Windows: `%LOCALAPPDATA%\warp\`

Each application gets its own folder in `packages/`, named after the executable's file name. The application itself is extracted into a `static` subfolder.

* `--prefix` sets a fixed folder name instead of the file name. Use it when the binary may be renamed (for example by version or by a browser download), so that all copies share one cache, or when two different applications have the same file name.
* `--unique-id` extracts every build into its own subfolder instead of `static`, and by default removes the folders of other builds on launch. With `--no-clean`, older builds are kept, so several versions can run side by side. It is no longer needed for reliable updates, since the cache is validated with a hash of the payload.

### Cleaning the Cache

Running a packed binary with `--warp-clean` as its only argument deletes its whole cache folder, including all `--unique-id` builds, and exits without starting the application:

```sh
./app --warp-clean
```

If the flag is combined with other arguments, everything is passed to the application as usual. While the application is running, the cleanup is refused. This check only covers applications packed with warp 1.4.0 or later.

---

## macOS Security Considerations

Recent versions of macOS include security mechanisms such as **Gatekeeper** and the **quarantine attribute**.

When distributing macOS binaries created with `warp-packer`, users may encounter warnings if the executable:

- is **not signed with an Apple Developer certificate**, or
- was **downloaded from the internet**, which automatically adds a quarantine attribute.

In some cases macOS may display messages such as:

- *“The application is damaged and can't be opened.”*
- *“This app cannot be opened because it is from an unidentified developer.”*

For local testing, the quarantine attribute can be removed using:

```sh
xattr -d com.apple.quarantine <binary>
````

Alternatively, developers distributing macOS applications should consider **proper code signing and notarization** using Apple's developer tools.

The build process used in this repository applies an **ad-hoc code signature** to the generated macOS binaries to improve compatibility, but this is not a replacement for official Apple code signing.

---

## Authors

* Original Author: Diego Giagio `<diego@giagio.com>`
* Modifications and enhancements by: [@kirbylink](https://github.com/kirbylink) and [@palexdev](https://github.com/palexdev)

---

## License

This project is licensed under the MIT License – see the [LICENSE](LICENSE) file for details.

---

## Who Uses Warp?

* [Buckaroo](https://github.com/loopperfect/buckaroo): C++ package manager
* [Buck (Warp)](https://github.com/njlr/buck-warp): Build system wrapper